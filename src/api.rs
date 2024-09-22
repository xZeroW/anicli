use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Error as SerdeError;
use thiserror::Error; // Import thiserror for custom error handling

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Request error: {0}")]
    Reqwest(#[from] ReqwestError),
    #[error("Serialization error: {0}")]
    Serde(#[from] SerdeError),
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub animes: Vec<Anime>,
    pub genres: Vec<String>,

    #[serde(rename = "currentPage")]
    pub current_page: u32,

    #[serde(rename = "hasNextPage")]
    pub has_next_page: bool,

    #[serde(rename = "totalPages")]
    pub total_pages: u32,
}

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub id: String,
    pub name: String,
    pub episodes: Episodes,
    pub duration: String,
    pub rated: bool,
}

#[derive(Debug, Deserialize)]
pub struct Episodes {
    pub eps: Option<u32>,
    pub sub: Option<u32>,
    pub dub: Option<u32>,
}

pub async fn search_anime(keyword: &str) -> Result<ApiResponse, ApiError> {
    let url = format!(
        "https://api-anime-rouge.vercel.app/aniwatch/search?keyword={}&page=1",
        keyword
    );
    let response = reqwest::get(&url).await?;
    let api_response = response.json::<ApiResponse>().await?;

    Ok(api_response)
}
