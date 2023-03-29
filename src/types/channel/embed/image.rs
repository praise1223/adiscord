use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
}
