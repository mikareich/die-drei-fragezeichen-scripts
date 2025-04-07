mod gemini;
mod metadata;
mod s3;
mod tracks;
mod transcript;

use futures::future::join_all;
use metadata::get_metadata;
use s3::store_track;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tracks::get_tracks;
use transcript::generate_transcript;

pub type EpisodeId = u16;

fn get_gemini_api_key() -> String {
    env::var("GEMINI_API_KEY").expect("gemini api key must be defined in .env")
}

fn get_absolute_path(relative_path: &str) -> PathBuf {
    let cwd = env::current_dir().unwrap();
    let suffix = Path::new(relative_path);
    cwd.join(suffix)
}

const MAX_CONCURRENT_PROCESSES: usize = 10;

#[tokio::main]
async fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();

    let env_path = get_absolute_path(args[0].as_str());
    dotenv::from_path(env_path).ok();

    let id: EpisodeId = args[1]
        .parse::<EpisodeId>()
        .expect("ID should be a valid episode number");

    let target_path = get_absolute_path(args[2].as_str());

    println!("👯 preprocessing episode...");
    let metadata = get_metadata(&id).await;
    let audio_tracks = get_tracks(&metadata, &target_path).await;

    println!("👯 starting transcription...");
    for (chunk_idx, chunk) in audio_tracks.chunks(MAX_CONCURRENT_PROCESSES).enumerate() {
        let handles = chunk.iter().enumerate().map(|(part, (file_name, buffer))| {
            let part = chunk_idx * MAX_CONCURRENT_PROCESSES + part;
            let metadata = metadata.clone();
            let buffer = buffer.clone();
            let file_name = file_name.clone();
            let transcript_path = target_path.join(format!("{file_name}.transcript.json"));

            tokio::spawn(async move {
                println!("✍️ transcribing #{part} ({file_name})");

                let transcript = match fs::read_to_string(&transcript_path) {
                    Ok(transcript) => transcript,
                    Err(_) => {
                        let transcript = generate_transcript(&metadata, &buffer).await;
                        let _ = fs::write(&transcript_path, &transcript);
                        transcript
                    }
                };

                println!("🪣 storing #{part} in s3");
                store_track(part, &metadata, &transcript).await;
            })
        });

        join_all(handles).await;
    }

    println!("✅ processed episode successfully");
}
