use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use sysinfo::{System, RefreshKind, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL};
use indicatif::{ProgressBar, ProgressStyle};
use tokio::time::sleep;

pub async fn show_progress(generated: Arc<AtomicU64>, total: u64) {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "🎁 {bar:40.green/black} {percent}% | {pos}/{len}\n🖥 CPU:{msg}"
        )
        .unwrap()
        .progress_chars("█-"), // The completed part is █, the unfinished part is -
    );

    // CPU Warm-up
    sys.refresh_cpu_usage();
    sleep(MINIMUM_CPU_UPDATE_INTERVAL).await;
    sys.refresh_cpu_usage();

    loop {
        let gen = generated.load(Ordering::SeqCst).min(total);
        pb.set_position(gen);

        sys.refresh_cpu_usage();
        sys.refresh_memory();

        let avg_cpu = if sys.cpus().is_empty() {
            0.0
        } else {
            sys.cpus().iter().map(|c| c.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32
        };
        let ram_percent = if sys.total_memory() == 0 {
            0.0
        } else {
            (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0
        };

        // Dynamically update msg with metrics
        pb.set_message(format!("{:.1}% | 💾 RAM:{:.1}%", avg_cpu, ram_percent));

        if gen >= total {
            // Switch to a one-line template without metrics before finishing
            pb.set_style(
                ProgressStyle::with_template(
                    "🎁 {bar:40.green/green} {percent}% | {pos}/{len}"
                )
                .unwrap()
                .progress_chars("█-"), // Keep the dashes
            );
            pb.set_message(""); // We remove metrics
            pb.finish(); // We finish by keeping the strip
            break;
        }

        sleep(Duration::from_millis(250)).await;
    }
}