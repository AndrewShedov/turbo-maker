use crate::config::{Config, Document, NumberThreads};
use crate::utils::get_cpu_info;
use mongodb::Client;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

pub async fn run_workers<F>(config: Config, generate_fn: F, generated: Arc<AtomicU64>)
where
    F: Fn(&Config, u64) -> Document + Send + Sync + Clone + 'static,
{
    let (cpu_count, _cpu_model) = get_cpu_info();
    let threads = match config.settings.number_threads {
        NumberThreads::Max => cpu_count,
        NumberThreads::Count(num) => num as usize,
    };

    let documents_per_thread = config.settings.number_documents / (threads as u64);
    let remainder = config.settings.number_documents % (threads as u64);
    let (tx, mut rx) = mpsc::channel(threads);

    for i in 0..threads {
        let extra = if i < (remainder as usize) { 1 } else { 0 };
        let from = (i as u64) * documents_per_thread;
        let to = from + documents_per_thread + extra;
        let config = config.clone();
        let generated = Arc::clone(&generated);
        let generate_fn = generate_fn.clone();
        let tx = tx.clone();

        tokio::spawn(async move {
            let client = Client::with_uri_str(&config.settings.uri).await.unwrap();
            let db = client.database(&config.settings.db);
            let collection = db.collection::<Document>(&config.settings.collection);

            let mut docs = Vec::with_capacity(config.settings.batch_size as usize);
            for i in from..to {
                let doc = generate_fn(&config, i * config.settings.time_step_ms);
                docs.push(doc);
                if docs.len() >= (config.settings.batch_size as usize) {
                    let docs_to_insert = std::mem::take(&mut docs);
                    if let Ok(_) = collection.insert_many(docs_to_insert, None).await {
                        generated.fetch_add(config.settings.batch_size as u64, Ordering::SeqCst);
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