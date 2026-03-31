use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Maps integer type IDs to human-readable names (mirrors ro.py's `asset_type_names`).
pub fn asset_type_name(type_id: u32) -> &'static str {
    match type_id {
        1 => "Image",
        2 => "T-Shirt",
        3 => "Audio",
        4 => "Mesh",
        5 => "Lua",
        6 => "HTML",
        7 => "Text",
        8 => "Hat",
        9 => "Place",
        10 => "Model",
        11 => "Shirt",
        12 => "Pants",
        13 => "Decal",
        16 => "Avatar",
        17 => "Head",
        18 => "Face",
        19 => "Gear",
        21 => "Badge",
        22 => "Group Emblem",
        24 => "Animation",
        25 => "Arms",
        26 => "Legs",
        27 => "Torso",
        28 => "Right Arm",
        29 => "Left Arm",
        30 => "Left Leg",
        31 => "Right Leg",
        32 => "Package",
        33 => "YouTubeVideo",
        34 => "Pass",
        35 => "App",
        37 => "Code",
        38 => "Plugin",
        39 => "SolidModel",
        40 => "MeshPart",
        41 => "HairAccessory",
        42 => "FaceAccessory",
        43 => "NeckAccessory",
        44 => "ShoulderAccessory",
        45 => "FrontAccessory",
        46 => "BackAccessory",
        47 => "WaistAccessory",
        48 => "ClimbAnimation",
        49 => "DeathAnimation",
        50 => "FallAnimation",
        51 => "IdleAnimation",
        52 => "JumpAnimation",
        53 => "RunAnimation",
        54 => "SwimAnimation",
        55 => "WalkAnimation",
        56 => "PoseAnimation",
        59 => "LocalizationTableManifest",
        60 => "LocalizationTableTranslation",
        61 => "EmoteAnimation",
        62 => "Video",
        63 => "TexturePack",
        64 => "TShirtAccessory",
        65 => "ShirtAccessory",
        66 => "PantsAccessory",
        67 => "JacketAccessory",
        68 => "SweaterAccessory",
        69 => "ShortsAccessory",
        70 => "LeftShoeAccessory",
        71 => "RightShoeAccessory",
        72 => "DressSkirtAccessory",
        73 => "FontFamily",
        76 => "EyebrowAccessory",
        77 => "EyelashAccessory",
        78 => "MoodAnimation",
        79 => "DynamicHead",
        _ => "Unknown",
    }
}

/// Identifies the creator of an asset.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum CreatorType {
    User,
    Group,
}

/// Represents a Roblox asset (item from the catalog or uploaded content).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EconomyAsset {
    pub id: u64,
    pub product_id: Option<u64>,
    pub name: String,
    pub description: String,
    pub asset_type_id: u32,
    pub is_new: Option<bool>,
    pub is_for_sale: bool,
    pub is_public_domain: bool,
    pub is_limited: bool,
    pub is_limited_unique: bool,
    pub remaining: Option<i64>,
    pub sales: Option<u64>,
    pub price: Option<i64>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub creator: Option<AssetCreator>,
}

impl EconomyAsset {
    /// Returns the human-readable name for this asset's type ID.
    pub fn asset_type_name(&self) -> &'static str {
        asset_type_name(self.asset_type_id)
    }
}

/// Represents the creator of an asset (user or group).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreator {
    pub id: u64,
    pub name: Option<String>,
    pub creator_type: Option<String>,
}
