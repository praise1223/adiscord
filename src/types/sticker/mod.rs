pub mod format;
pub mod item;
pub mod r#type;

use self::{format::StickerFormatType, r#type::StickerType};
use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Sticker {
    /// id of the sticker
    pub id: String,

    /// for standard stickers, id of the pack the sticker is from
    pub pack_id: Option<String>,

    /// name of the sticker
    pub name: String,

    /// description of the sticker
    pub description: String,

    /// autocomplete/suggestion tags for the sticker (max 200 characters)
    pub tags: String,

    /// Deprecated previously the sticker asset hash, now an empty string
    #[deprecated]
    pub asset: Option<String>,

    #[serde(rename = "type")]

    /// type of sticker
    pub r#type: StickerType,

    /// type of sticker format
    pub format_type: StickerFormatType,

    /// whether this guild sticker can be used, may be false due to loss of Server Boosts
    pub available: Option<bool>,

    /// id of the guild that owns this sticker
    pub guild_id: Option<String>,

    /// the user that uploaded the guild sticker
    pub user: Option<User>,

    /// the standard sticker's sort order within its pack
    pub sort_value: Option<i128>,
}
