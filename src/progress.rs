// progress.rs
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use sysinfo::{CpuRefreshKind, RefreshKind, System, MINIMUM_CPU_UPDATE_INTERVAL};
use tokio::time::sleep;

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

pub async fn show_progress(generated: Arc<AtomicU64>, total: u64) {
    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    // CPU warm-up
    sys.refresh_cpu_usage();
    sleep(MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();

    let bar_width = 40;

    loop {
        let gen = generated.load(Ordering::SeqCst).min(total);
        let percent = ((gen as f64) / (total as f64)) * 100.0;
        let filled = ((percent / 100.0) * (bar_width as f64)).round() as usize;

        let formatted_pos = format_with_commas(gen);
        let formatted_len = format_with_commas(total);

        let bar = format!(
            "\x1B[32m{}\x1B[0m{}",
            "█".repeat(filled),
            "-".repeat(bar_width - filled)
        );

        // updating system metrics
        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let avg_cpu = if sys.cpus().is_empty() {
            0.0
        } else {
            sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / (sys.cpus().len() as f32)
        };
        let ram_percent = if sys.total_memory() == 0 {
            0.0
        } else {
            ((sys.used_memory() as f64) / (sys.total_memory() as f64)) * 100.0
        };

        // redraw (clear 2 lines)
        print!("\x1B[2F\x1B[2K"); // cursor 2 lines up, clear
        println!(
            "🎁 {}  {:3.0}% | {} / {}",
            bar, percent, formatted_pos, formatted_len
        );
        println!("           CPU:{:5.1}% | RAM:{:5.1}%", avg_cpu, ram_percent);

        if gen >= total {
            // final: clear CPU/RAM lines and leave only the bar
            print!("\x1B[1F\x1B[2K");
            print!("\x1B[1F\x1B[2K");
            println!(
                "🎁 \x1B[32m{}\x1B[0m  100% | {} / {}",
                "█".repeat(bar_width),
                formatted_len,
                formatted_len
            );
            break;
        }

        sleep(Duration::from_millis(250)).await;
    }
}
