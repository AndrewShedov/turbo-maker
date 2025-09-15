use clap::Parser;
use turbo_maker::worker::run_workers;
use turbo_maker::progress::show_progress;
use turbo_maker::config::load_and_validate_config;
use turbo_maker::generate::generate_document;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "./turbo-maker.config.toml")]
    config_path: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let config = load_and_validate_config(&cli.config_path).await.unwrap();
    let generated = Arc::new(AtomicU64::new(0));

    tokio::spawn(show_progress(Arc::clone(&generated), config.number_documents));

    run_workers(config, generate_document, generated).await;
}