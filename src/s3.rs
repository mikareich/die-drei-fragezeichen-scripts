use crate::metadata::schema::EpisodeMetadata;
use aws_config::load_from_env;
use aws_sdk_s3::{primitives::ByteStream, Client};

const BUCKET_NAME: &str = "die-drei-fragezeichen-scripts";

pub async fn store_track(part: usize, metadata: &EpisodeMetadata, track: &String) {
    let conf = load_from_env().await;
    let client = Client::new(&conf);
    let key = format!("json/{}/{}.json", metadata.number, part);

    let _ = client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(key)
        .body(ByteStream::from(track.clone().into_bytes()))
        .send()
        .await;
}
