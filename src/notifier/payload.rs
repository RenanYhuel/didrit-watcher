use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NtfyAction {
    pub action: String,
    pub label: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtfyPayload {
    pub topic: String,
    pub title: String,
    pub message: String,
    pub priority: u8,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<NtfyAction>,
    pub markdown: bool,
}

impl NtfyPayload {
    pub fn new(topic: &str, title: &str, message: &str) -> Self {
        Self {
            topic: topic.to_string(),
            title: title.to_string(),
            message: message.to_string(),
            priority: 3,
            tags: Vec::new(),
            click: None,
            attach: None,
            filename: None,
            actions: Vec::new(),
            markdown: true,
        }
    }

    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = priority.clamp(1, 5);
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_click(mut self, url: &str) -> Self {
        self.click = Some(url.to_string());
        self
    }

    pub fn with_attachment(mut self, url: &str, filename: Option<&str>) -> Self {
        self.attach = Some(url.to_string());
        self.filename = filename.map(|f| f.to_string());
        self
    }

    pub fn add_action(mut self, label: &str, url: &str) -> Self {
        if self.actions.len() < 3 {
            self.actions.push(NtfyAction {
                action: "view".to_string(),
                label: label.to_string(),
                url: url.to_string(),
                clear: Some(false),
            });
        }
        self
    }
}
