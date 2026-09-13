use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttachmentLink {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CahierEntry {
    pub raw_text: String,
    pub links: Vec<AttachmentLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DevoirEntry {
    pub raw_text: String,
    pub links: Vec<AttachmentLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoursEntry {
    pub title: String,
    pub links: Vec<AttachmentLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParsedPage {
    pub source_url: String,
    pub page_title: String,
    pub cahier_entries: Vec<CahierEntry>,
    pub devoir_entries: Vec<DevoirEntry>,
    pub cours_entries: Vec<CoursEntry>,
    pub all_attachments: Vec<AttachmentLink>,
    pub full_content_hash: String,
}
