use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use mongodb::bson::Document as BsonDocument;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    #[serde(rename = "settings")]
    pub settings: Settings,
    #[serde(default)]
    pub document_fields: Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub uri: String,
    pub db: String,
    pub collection: String,
    #[serde(deserialize_with = "deserialize_number_threads")]
    pub number_threads: NumberThreads,
    pub number_documents: u64,
    pub batch_size: u64,
    pub time_step_ms: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberThreads {
    Max,
    Count(u64),
}

fn deserialize_number_threads<'de, D>(deserializer: D) -> Result<NumberThreads, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        Value::String(s) if s.to_lowercase() == "max" => Ok(NumberThreads::Max),
        Value::Number(n) => {
            if let Some(num) = n.as_u64() {
                if num > 0 {
                    Ok(NumberThreads::Count(num))
                } else {
                    Err(serde::de::Error::custom("number_threads must be a positive integer or 'max'"))
                }
            } else {
                Err(serde::de::Error::custom("number_threads must be a positive integer or 'max'"))
            }
        }
        _ => Err(serde::de::Error::custom("number_threads must be a positive integer or 'max'")),
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct Document {
    #[serde(flatten)]
    pub custom: BsonDocument,
}

pub async fn load_and_validate_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let mut contents = String::new();
    let mut file = File::open(path).await?;
    file.read_to_string(&mut contents).await?;

    let config: Config = toml::from_str(&contents)?;
    validate_config(&config)?;
    Ok(config)
}

fn validate_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let required_fields = [
        "uri",
        "db",
        "collection",
        "number_documents",
        "batch_size",
        "time_step_ms",
    ];
    for field in required_fields {
        if !config_has_field(&config.settings, field) {
            return Err(format!("Missing required config field: {}", field).into());
        }
    }

    // Validation of document_fields
    if let Some(fields) = config.document_fields.as_object() {
        for (key, value) in fields {
            if value.is_object() {
                let obj = value.as_object().unwrap();
                if obj.contains_key("function") {
                    let func_type = obj.get("function").unwrap().as_str().ok_or_else(|| {
                        format!("Field '{}' has invalid 'function' value, expected string", key)
                    })?;
                    match func_type {
                        "generate_long_string" => {
                            if !obj.contains_key("length") {
                                return Err(format!(
                                    "Field '{}' with function 'generate_long_string' requires 'length' parameter",
                                    key
                                ).into());
                            }
                            if !obj.get("length").unwrap().is_i64() && !obj.get("length").unwrap().is_u64() {
                                return Err(format!(
                                    "Field '{}' has invalid 'length' value, expected integer",
                                    key
                                ).into());
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Field '{}' has unknown function type '{}'",
                                key, func_type
                            ).into());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn config_has_field(settings: &Settings, field: &str) -> bool {
    match field {
        "uri" => !settings.uri.is_empty(),
        "db" => !settings.db.is_empty(),
        "collection" => !settings.collection.is_empty(),
        "number_documents" => settings.number_documents > 0,
        "batch_size" => settings.batch_size > 0,
        "time_step_ms" => true,
        _ => false,
    }
}