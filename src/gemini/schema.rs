use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeminiResponse {
    #[serde(rename = "candidates")]
    pub candidates: Vec<Candidate>,
    #[serde(rename = "usageMetadata")]
    pub usage_metadata: UsageMetadata,
    #[serde(rename = "modelVersion")]
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Candidate {
    #[serde(rename = "content")]
    pub content: Content,
    #[serde(rename = "finishReason")]
    pub finish_reason: String,
    #[serde(rename = "avgLogprobs")]
    pub avg_logprobs: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    #[serde(rename = "parts")]
    pub parts: Vec<Part>,
    #[serde(rename = "role")]
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Part {
    #[serde(rename = "text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsageMetadata {
    #[serde(rename = "promptTokenCount")]
    pub prompt_token_count: i64,
    #[serde(rename = "candidatesTokenCount")]
    pub candidates_token_count: i64,
    #[serde(rename = "totalTokenCount")]
    pub total_token_count: i64,
    #[serde(rename = "promptTokensDetails")]
    pub prompt_tokens_details: Vec<PromptTokensDetail>,
    #[serde(rename = "candidatesTokensDetails")]
    pub candidates_tokens_details: Vec<CandidatesTokensDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PromptTokensDetail {
    #[serde(rename = "modality")]
    pub modality: String,
    #[serde(rename = "tokenCount")]
    pub token_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CandidatesTokensDetail {
    #[serde(rename = "modality")]
    pub modality: String,
    #[serde(rename = "tokenCount")]
    pub token_count: i64,
}
