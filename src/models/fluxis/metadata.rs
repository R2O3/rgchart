use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(alias = "Title")]
    pub title: String,
    
    #[serde(rename = "title-rm", alias = "Title-rm", alias = "Title-Rm", alias = "TITLE-RM", skip_serializing_if = "Option::is_none")]
    pub title_rm: Option<String>,
    
    #[serde(alias = "Artist")]
    pub artist: String,
    
    #[serde(rename = "artist-rm", alias = "Artist-rm", alias = "Artist-Rm", alias = "ARTIST-RM", skip_serializing_if = "Option::is_none")]
    pub artist_rm: Option<String>,
    
    #[serde(alias = "Mapper")]
    pub mapper: String,
    
    #[serde(alias = "Difficulty")]
    pub difficulty: String,
    
    #[serde(alias = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    #[serde(rename = "bg-source", alias = "Bg-source", alias = "BG-Source", alias = "BG-SOURCE", skip_serializing_if = "Option::is_none")]
    pub bg_source: Option<String>,
    
    #[serde(rename = "cover-source", alias = "Cover-source", alias = "Cover-Source", alias = "COVER-SOURCE", skip_serializing_if = "Option::is_none")]
    pub cover_source: Option<String>,
    
    #[serde(alias = "Tags")]
    pub tags: String,
    
    #[serde(alias = "Previewtime", alias = "PreviewTime")]
    pub previewtime: i32,
}

impl Metadata {
    pub fn display_title(&self) -> String {
        let title_rm = self.title_rm.clone();
        if !title_rm.unwrap_or("".to_string()).is_empty() {
            self.title_rm.clone().unwrap()
        } else {
            self.title.clone()
        }
    }
    
    pub fn display_artist(&self) -> String {
        let artist_rm = self.artist_rm.clone();
        if !artist_rm.unwrap_or("".to_string()).is_empty() {
            self.artist_rm.clone().unwrap()
        } else {
            self.artist.clone()
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            title: String::new(),
            title_rm: Some(String::new()),
            artist: String::new(),
            artist_rm: Some(String::new()),
            mapper: String::new(),
            difficulty: String::new(),
            source: Some(String::new()),
            bg_source: Some(String::new()),
            cover_source: Some(String::new()),
            tags: String::new(),
            previewtime: 0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    #[serde(alias = "Accent")]
    pub accent: String,
    
    #[serde(alias = "Primary")]
    pub primary: String,
    
    #[serde(alias = "Secondary")]
    pub secondary: String,
    
    #[serde(alias = "Middle")]
    pub middle: String,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            accent: String::new(),
            primary: String::new(),
            secondary: String::new(),
            middle: String::new(),
        }
    }
}