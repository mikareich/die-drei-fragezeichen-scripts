pub mod schema;

use crate::EpisodeId;
use schema::EpisodeMetadata;

fn format_id(id: &EpisodeId) -> String {
    let id = id.to_string().chars().collect::<Vec<char>>();

    let mut formatted_id = vec!['0', '0', '0'];
    formatted_id.extend(&id);

    let formatted_id = String::from_iter(&formatted_id[(formatted_id.len() - 3)..]);

    formatted_id
}

pub async fn get_metadata(id: &EpisodeId) -> EpisodeMetadata {
    let formatted_id = format_id(id);
    let url = format!("https://dreimetadaten.de/data/Serie/{formatted_id}/metadata.json");

    let response = reqwest::get(url).await.unwrap();
    let body = response.text().await.unwrap();

    serde_json::from_str::<EpisodeMetadata>(body.as_str())
        .expect("metadata json should conform the schema")
}
