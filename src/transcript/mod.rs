use serde_json::json;

use crate::gemini::files::{remove_file, upload_file};
use crate::gemini::send_prompt;
use crate::{get_absolute_path, metadata::schema::EpisodeMetadata};
use std::fs;

const PROMPT_PATH: &str = "src/transcript/prompt.txt";

/// generates transcript of audio buffer
pub async fn generate_transcript(metadata: &EpisodeMetadata, buffer: &Vec<u8>) -> String {
    let intro_path = get_absolute_path("src/transcript/intro.mp3");
    let intro_buffer = fs::read(intro_path).unwrap();

    // upload audio files

    let intro_audio = upload_file("audio/mp3", &intro_buffer).await;
    let episode_audio = upload_file("audio/mp3", &buffer).await;
    let audio_files = vec![&intro_audio, &episode_audio];

    // generate prompt

    let prompt_base = fs::read_to_string(PROMPT_PATH).unwrap();
    let metadata_json = serde_json::to_value(metadata).unwrap();
    let prompt = format!(
        "{prompt_base} ```json\n{metadata_json}``` intro id: {}",
        &intro_audio.name
    );

    // generate output schema

    let character_pattern = &metadata
        .roles
        .iter()
        .flat_map(|role| [role.role.replace(".", "\\."), String::from("|")])
        .collect::<String>();

    let role_pattern = format!("\\character:(Unknown|{character_pattern})\\|\\ambient\\");

    let oas = json!({
      "type": "object",
      "properties": {
        "script": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "role": {
                "type": "string",
                "pattern": role_pattern
              },
              "content": {
                "type": "string"
              }
            },
            "required": [
              "role",
              "content"
            ]
          }
        }
      },
      "required": [
        "script"
      ]
    });

    let transcript = send_prompt(prompt, Some(audio_files), Some(oas)).await;

    // remove audio files again

    remove_file(intro_audio).await;
    remove_file(episode_audio).await;

    transcript.to_string()
}
