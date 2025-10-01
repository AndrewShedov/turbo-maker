use crate::config::{Config, Document};
use crate::functions;
use mongodb::bson::{Document as BsonDocument, DateTime as BsonDateTime};
use serde_json::Value;

pub fn generate_document(config: &Config, offset_ms: i64) -> Document {
    let now = chrono::Utc::now();
    let timestamp = if config.settings.time_step_ms > 0 {
        now.timestamp_millis() + offset_ms
    } else {
        now.timestamp_millis()
    };
    // Creating bson::DateTime for MongoDB
    let bson_date = BsonDateTime::from_millis(timestamp);

    let default_map = serde_json::Map::new();
    let document_fields = config.document_fields.as_object().unwrap_or(&default_map);

    let mut custom_data = BsonDocument::new();

    for (key, value) in document_fields {
        match value {
            Value::Object(obj) if obj.contains_key("function") => {
                let func_type = obj.get("function").unwrap().as_str().unwrap();
                match func_type {
                    "generate_long_string" => {
                        if let Some(length_val) = obj.get("length") {
                            let length = length_val.as_u64().or_else(|| length_val.as_i64().map(|v| v as u64));
                            if let Some(length) = length {
                                let result = functions::generate_long_string(length as usize);
                                custom_data.insert(key.clone(), result);
                            }
                        }
                    }
                    _ => {
                        // Ignore unknown functions; validation has already occurred in config.rs
                    }
                }
            }
            Value::String(s) => {
                if key == "created_at" || key == "updated_at" {
                    if s.is_empty() {
                        custom_data.insert(key.clone(), bson_date);
                    } else {
                        // Custom name for the date (e.g. createdAt, updatedAt)
                        custom_data.insert(s.clone(), bson_date);
                    }
                } else {
                    custom_data.insert(key.clone(), s.clone());
                }
            }
            Value::Null => {
                if key == "created_at" || key == "updated_at" {
                    custom_data.insert(key.clone(), bson_date);
                }
            }
            _ => {
                // For other types, convert serde_json::Value to bson
                if let Ok(bson_value) = mongodb::bson::to_bson(value) {
                    custom_data.insert(key.clone(), bson_value);
                }
            }
        }
    }

    Document {
        custom: custom_data,
    }
}