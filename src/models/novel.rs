use super::{user::User, *};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Response {
    pub novels: Vec<Novel>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub restrict: i64,
    pub x_restrict: i64,
    pub image_urls: ImageUrls,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub page_count: i64,
    pub text_length: i64,
    pub user: User,
    #[serde(deserialize_with = "ok_or_none")]
    pub series: Option<Series>, // `series` can be {}
    pub is_bookmarked: bool,
    pub total_bookmarks: i64,
    pub total_view: i64,
    pub visible: bool,
    pub total_comments: i64,
    pub is_muted: bool,
    pub is_mypixiv_only: bool,
    pub is_x_restricted: bool,
}

use serde::Deserializer;
fn ok_or_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(T::deserialize(deserializer).ok())
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct NovelTextResponse {
    // novel_marker: NovelMarker,
    pub novel_text: String,
    #[serde(deserialize_with = "ok_or_none")]
    pub series_prev: Option<Novel>,
    #[serde(deserialize_with = "ok_or_none")]
    pub series_next: Option<Novel>,
}
