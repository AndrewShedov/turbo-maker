use clap::Parser;
use turbo_maker::worker::run_workers;
use turbo_maker::progress::show_progress;
use turbo_maker::config::load_and_validate_config;
use turbo_maker::generate::generate_document;
use turbo_maker::utils::get_cpu_info;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::time::Instant;
use sysinfo::{ System, RefreshKind, MemoryRefreshKind };

// Function to format number with commas
fn format_with_commas(num: u64) -> String {
    let s = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in s.chars().rev() {
        if count == 3 {
            result.insert(0, ',');
            count = 0;
        }
        result.insert(0, c);
        count += 1;
    }
    result
}

// Function to get RAM information
fn get_ram_info() -> String {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::everything())
    );
    sys.refresh_memory();
    let total_memory = sys.total_memory(); // in bytes
    let memory_gb = (total_memory as f64) / (1024.0 * 1024.0 * 1024.0); // Convert to GB
    format!("{:.1} GB", memory_gb)
}

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

    let (cpu_count, cpu_model) = get_cpu_info();
    let threads = if
        config.number_threads == "max" ||
        config.number_threads.parse::<usize>().is_err()
    {
        cpu_count
    } else {
        config.number_threads.parse().unwrap_or(cpu_count)
    };

    // Display system and configuration information with an initial newline for spacing
    println!("\n🖥️ CPU: {} | {} threads", cpu_model, cpu_count);
    println!("   RAM: {}\n", get_ram_info());
    println!(
        "🚀 Start | {} threads | {} documents | {} batch | {} timeStepMs\n",
        threads,
        format_with_commas(config.number_documents),
        format_with_commas(config.batch_size),
        format_with_commas(config.time_step_ms)
    );
    println!(
        "🌐 URI: {}\n🗄️ Database: {}\n📂 Collection: {}\n", // Added extra \n for spacing before progress
        config.uri,
        config.db,
        config.collection
    );
    println!("\n");
    // Launching a progress bar in a separate task
    let progress_handle = tokio::spawn(
        show_progress(Arc::clone(&generated), config.number_documents)
    );

    // Measuring time
    let start = Instant::now();

    // Start generation with config cloning
    run_workers(config.clone(), generate_document, generated.clone()).await;

    // Wait until the progress bar is complete
    if let Ok(()) = progress_handle.await {
        // Completion and output of statistics
        let end = start.elapsed();
        let duration_ms = end.as_millis() as f64;

        let total_generated = generated.load(std::sync::atomic::Ordering::SeqCst);
        let formatted_total = format_with_commas(total_generated);

        let hours = (duration_ms / 3600000.0) as u64;
        let minutes = ((duration_ms % 3600000.0) / 60000.0) as u64;
        let seconds = ((duration_ms % 60000.0) / 1000.0) as u64;
        let milliseconds = (duration_ms % 1000.0) as u64;

        let formatted_duration = if hours > 0 {
            format!("{} hr {} min", hours, minutes)
        } else if minutes > 0 {
            format!("{} min {} sec {} ms", minutes, seconds, milliseconds)
        } else if seconds > 0 {
            format!("{} sec {} ms", seconds, milliseconds)
        } else {
            format!("{} ms", milliseconds)
        };

        let duration_sec = duration_ms / 1000.0;
        let speed = ((total_generated as f64) / duration_sec) as u64;
        let formatted_speed = format_with_commas(speed);
        let per_document_ms = duration_ms / (total_generated as f64);
        println!("");
        println!("✅ Successfully created: {} documents.", formatted_total);
        println!("⏱️ Creation time: {}", formatted_duration);
        println!("⚡ Speed: {} documents/sec.", formatted_speed);
        println!("📊 Average time per document: {:.5} ms", per_document_ms);

        println!("👋 Completion of work...");
    }
}
