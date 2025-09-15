use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn generate_document(offset_ms: u64) -> crate::config::Document {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64 + offset_ms;
    let mut rng = rand::thread_rng();
    crate::config::Document {
        title: "example".to_string(),
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