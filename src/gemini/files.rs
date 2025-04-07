use crate::get_gemini_api_key;
use reqwest::Client;
use uuid::Uuid;

#[derive(Debug)]
pub struct MediaFile {
    pub mime_type: String,
    pub name: String,
    pub uri: String,
}

/// uploads files to geminis file bucket
pub async fn upload_file(mime_type: &str, buffer: &Vec<u8>) -> MediaFile {
    let api_key = get_gemini_api_key();

    let url =
        format!("https://generativelanguage.googleapis.com/upload/v1beta/files?key={api_key}");

    let num_bytes = buffer.len();

    let display_name = Uuid::new_v4();
    let body = format!("{{'file': {{'display_name': '{display_name}'}}}}");

    let client = Client::new();
    let response = client
        .post(url)
        .header("X-Goog-Upload-Protocol", "resumable")
        .header("X-Goog-Upload-Command", "start")
        .header("X-Goog-Upload-Header-Content-Length", num_bytes.to_string())
        .header("X-Goog-Upload-Header-Content-Type", "audio/mp3")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .unwrap();

    let upload_url = response
        .headers()
        .get("x-goog-upload-url")
        .expect("should contain the upload url")
        .to_str()
        .unwrap();

    // upload buffer
    let response = client
        .post(upload_url)
        .header("Content-Length", num_bytes.to_string())
        .header("X-Goog-Upload-Offset", "0")
        .header("X-Goog-Upload-Command", "upload, finalize")
        .body(buffer.clone())
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    let file_info = serde_json::from_str::<serde_json::Value>(body.as_str())
        .expect("file upload response should resolve to schema");

    let uri = file_info["file"]["uri"]
        .as_str()
        .expect("File uri should be returned");

    let name = file_info["file"]["name"]
        .as_str()
        .expect("File name should be returned");

    MediaFile {
        name: String::from(name),
        uri: String::from(uri),
        mime_type: String::from(mime_type),
    }
}

/// removes file from geminis file bucket
pub async fn remove_file(file: MediaFile) {
    let api_key = get_gemini_api_key();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/{}?key={api_key}",
        file.name
    );

    let client = Client::new();
    let _ = client.delete(url).send().await;
}
