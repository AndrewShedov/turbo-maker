use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub async fn show_progress(generated: std::sync::Arc<AtomicU64>, total: u64) {
    loop {
        let gen = generated.load(Ordering::SeqCst);
        let progress = gen as f64 / total as f64;
        let bar = "█".repeat((progress * 40.0) as usize) + &"-".repeat(40 - (progress * 40.0) as usize);
        print!("\r🎁 [{}] {:.1}% | {}/{}", bar, progress * 100.0, gen, total);
        if gen >= total { println!(); break; }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}