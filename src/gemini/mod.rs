pub mod files;
mod schema;

use crate::get_gemini_api_key;
use files::MediaFile;
use reqwest::Client;
use schema::GeminiResponse;
use serde_json::{json, Value};

const MODEL: &str = "gemini-2.0-flash-lite";

/// sends prompt to gemini-2.0-flash-lite
pub async fn send_prompt(
    prompt: String,
    files: Option<Vec<&MediaFile>>,
    oas: Option<Value>,
) -> String {
    let files = files.unwrap_or(vec![]);
    let api_key = get_gemini_api_key();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{MODEL}:generateContent?key={api_key}"
    );

    let mut body = json!({
        "contents": [{
            "parts": []
        }]
    });

    if let Some(oas) = oas {
        body["generationConfig"] = json!({
            "response_mime_type": "application/json",
            "response_schema": oas
        })
    }

    if let Some(parts) = body["contents"][0]["parts"].as_array_mut() {
        parts.push(json!({"text": prompt}));
        files.iter().for_each(|file| {
            let part = json!({"file_data": {"mime_type": file.mime_type, "file_uri": file.uri}});
            parts.push(part);
        });
    }

    let request_body = body.to_string();

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await
        .unwrap();

    let response_body = response.text().await.unwrap();
    let response_body: GeminiResponse =
        serde_json::from_str::<GeminiResponse>(response_body.as_str())
            .expect("geminis response should resolve to schema");

    response_body
        .candidates
        .into_iter()
        .flat_map(|content| content.content.parts)
        .map(|part| part.text)
        .collect::<String>()
}
