use crate::config::Config;
use crate::config::Document;
use crate::utils::get_cpu_info;
use mongodb::Client;
use tokio::sync::mpsc;
use std::sync::Arc;
use std::sync::atomic::{ AtomicU64, Ordering };

pub async fn run_workers(
    config: Config,
    generate_fn: fn(u64) -> Document,
    generated: Arc<AtomicU64>
) {
    let (cpu_count, _cpu_model) = get_cpu_info();
    let threads = if
        config.number_threads == "max" ||
        config.number_threads.parse::<usize>().is_err()
    {
        cpu_count
    } else {
        config.number_threads.parse().unwrap_or(cpu_count)
    };

    let documents_per_thread = config.number_documents / (threads as u64);
    let remainder = config.number_documents % (threads as u64);
    let (tx, mut rx) = mpsc::channel(threads);

    for i in 0..threads {
        let extra = if i < (remainder as usize) { 1 } else { 0 };
        let from = (i as u64) * documents_per_thread;
        let to = from + documents_per_thread + extra;
        let config = config.clone();
        let generated = Arc::clone(&generated);
        let generate_fn = generate_fn;
        let tx = tx.clone();

        tokio::spawn(async move {
            let client = Client::with_uri_str(&config.uri).await.unwrap();
            let db = client.database(&config.db);
            let collection = db.collection::<Document>(&config.collection);

            let mut docs = Vec::with_capacity(config.batch_size as usize);
            for i in from..to {
                let doc = generate_fn(i * config.time_step_ms);
                docs.push(doc);
                if docs.len() >= (config.batch_size as usize) {
                    let docs_to_insert = std::mem::take(&mut docs);
                    if let Ok(_) = collection.insert_many(docs_to_insert, None).await {
                        generated.fetch_add(config.batch_size as u64, Ordering::SeqCst); // Обновляем на batch_size
                    }
                }
            }
            if !docs.is_empty() {
                let docs_to_insert = std::mem::take(&mut docs);
                let len = docs_to_insert.len() as u64;
                if let Ok(_) = collection.insert_many(docs_to_insert, None).await {
                    generated.fetch_add(len, Ordering::SeqCst);
                }
            }
            tx.send(()).await.unwrap();
        });
    }

    drop(tx);
    while rx.recv().await.is_some() {}
}
