use crate::{gemini::send_prompt, metadata::schema::EpisodeMetadata};
use std::{collections::HashMap, fs, path::PathBuf};

/// analyzes mp3 files in target folder, orders them and returns their buffer
pub async fn get_tracks(metadata: &EpisodeMetadata, path: &PathBuf) -> Vec<(String, Vec<u8>)> {
    let folder = fs::read_dir(path).unwrap();

    let mut audio_files: HashMap<String, Vec<u8>> = HashMap::new();

    for file in folder {
        let file = file.unwrap();
        let name = file.file_name().into_string().unwrap();

        if !name.ends_with(".mp3") {
            continue;
        }

        let path = file.path();
        let buffer = fs::read(path).unwrap();

        audio_files.insert(name, buffer);
    }
    let file_names = audio_files
        .keys()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n");

    let prompt_base = fs::read_to_string("src/tracks/prompt.txt").unwrap();
    let metadata_json = serde_json::to_value(metadata).unwrap();
    let prompt = format!("{prompt_base} {metadata_json} {file_names}");

    let response = send_prompt(prompt, None, None).await;

    let mut buffers: Vec<(String, Vec<u8>)> = vec![];

    for file_name in response.lines() {
        if let Some(buffer) = audio_files.get(file_name) {
            buffers.push((file_name.to_string(), buffer.clone()))
        }
    }

    buffers
}
