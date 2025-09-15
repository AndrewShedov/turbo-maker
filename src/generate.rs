use crate::config::Document;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn generate_document(offset_ms: u64) -> Document {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64 + offset_ms;
    let mut rng = rand::thread_rng();
    Document {
        title: "1111".to_string(),
        text: "example".to_string(),
        hashtags: vec!["#test".to_string()],
        views: rng.gen_range(120..=3125),
        main_image: "avatar_default.jpg".to_string(),
        liked: vec!["user1".to_string(), "user2".to_string()],
        user: "user123".to_string(),
        created_at: timestamp,
        updated_at: timestamp,
    }
}