use serde::Deserialize;
use serde::Serialize;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub uri: String,
    pub db: String,
    pub collection: String,
    pub number_threads: String,
    pub number_documents: u64,
    pub batch_size: u64,
    pub time_step_ms: u64,
    // Removen "generator_path"
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Document {
    pub title: String,
    pub text: String,
    pub hashtags: Vec<String>,
    pub views: u32,
    pub main_image: String,
    pub liked: Vec<String>,
    pub user: String,
    pub created_at: u64,
    pub updated_at: u64,
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
        "uri", "db", "collection", "number_threads", "number_documents", "batch_size", "time_step_ms",
        // Removen "generator_path"
    ];
    for field in required_fields {
        if !config_has_field(config, field) {
            return Err(format!("Missing required config field: {}", field).into());
        }
    }
    Ok(())
}

fn config_has_field(config: &Config, field: &str) -> bool {
    match field {
        "uri" => !config.uri.is_empty(),
        "db" => !config.db.is_empty(),
        "collection" => !config.collection.is_empty(),
        "number_threads" => !config.number_threads.is_empty(),
        "number_documents" => config.number_documents > 0,
        "batch_size" => config.batch_size > 0,
        "time_step_ms" => config.time_step_ms > 0,
        // Removen "generator_path"
        _ => false,
    }
}