use crate::error::Result;
use reqwest::Client as HttpClient;
use serde::{Deserialize, de::DeserializeOwned};

/// Sort order for paginated requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl SortOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            SortOrder::Ascending => "Asc",
            SortOrder::Descending => "Desc",
        }
    }
}

impl Default for SortOrder {
    fn default() -> Self {
        SortOrder::Ascending
    }
}

/// Represents a single page of results from a Roblox cursor-paginated endpoint.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    /// Cursor for the next page. `None` means this is the last page.
    pub next_page_cursor: Option<String>,
    /// Cursor for the previous page.
    pub previous_page_cursor: Option<String>,
    /// The items in this page.
    pub data: Vec<T>,
}

/// An async page iterator that automatically follows cursors to collect all items.
///
/// Use [`PageIterator::all`] to eagerly collect every item across all pages, or
/// [`PageIterator::next_page`] to manually advance one page at a time.
pub struct PageIterator<T> {
    client: HttpClient,
    url: String,
    page_size: u32,
    sort_order: SortOrder,
    max_items: Option<usize>,
    cursor: Option<String>,
    done: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> PageIterator<T> {
    /// Create a new `PageIterator`.
    ///
    /// - `client` — the underlying HTTP client (should carry session cookies).
    /// - `url` — the endpoint URL (without cursor/sort params).
    /// - `page_size` — items per page (10, 25, 50, or 100 depending on endpoint).
    /// - `sort_order` — ascending or descending.
    /// - `max_items` — optional cap on total items returned.
    pub fn new(
        client: HttpClient,
        url: impl Into<String>,
        page_size: u32,
        sort_order: SortOrder,
        max_items: Option<usize>,
    ) -> Self {
        Self {
            client,
            url: url.into(),
            page_size,
            sort_order,
            max_items,
            cursor: None,
            done: false,
            _marker: std::marker::PhantomData,
        }
    }

    /// Fetches the next page of results. Returns `None` when there are no more pages.
    pub async fn next_page(&mut self) -> Result<Option<Vec<T>>> {
        if self.done {
            return Ok(None);
        }

        let mut req = self
            .client
            .get(&self.url)
            .query(&[
                ("limit", self.page_size.to_string()),
                ("sortOrder", self.sort_order.as_str().to_string()),
            ]);

        if let Some(cursor) = &self.cursor {
            req = req.query(&[("cursor", cursor.as_str())]);
        }

        let response = req.send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::RobloxError::from_status(status, &body));
        }

        let page: Page<T> = response.json().await?;
        self.cursor = page.next_page_cursor.clone();

        if page.next_page_cursor.is_none() {
            self.done = true;
        }

        Ok(Some(page.data))
    }

    /// Eagerly collects all items across all pages into a `Vec<T>`.
    ///
    /// Respects `max_items` if set.
    pub async fn all(mut self) -> Result<Vec<T>> {
        let mut items = Vec::new();

        loop {
            match self.next_page().await? {
                None => break,
                Some(page_items) => {
                    items.extend(page_items);

                    if let Some(max) = self.max_items {
                        if items.len() >= max {
                            items.truncate(max);
                            break;
                        }
                    }

                    if self.done {
                        break;
                    }
                }
            }
        }

        Ok(items)
    }
}
