use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiGenerationConfig {
    #[serde(rename = "responseMimeType")]
    pub response_mime_type: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiRequestBody {
    pub contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseCandidatePart {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseCandidateContent {
    pub parts: Option<Vec<GeminiResponseCandidatePart>>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseCandidate {
    pub content: Option<GeminiResponseCandidateContent>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseBody {
    pub candidates: Option<Vec<GeminiResponseCandidate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AiAnalysis {
    pub title: String,
    pub fait_en_classe: Option<String>,
    pub a_faire: Option<String>,
    pub date_echeance: Option<String>,
    pub evaluation: Option<String>,
    pub nouveaux_documents: Vec<String>,
    pub priority: u8,
}
