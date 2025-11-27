use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub struct FileWatcher {
    watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn new(path: String, tx: Sender<String>) -> anyhow::Result<Self> {
        let tx = Arc::new(tx);
        
        let mut watcher = RecommendedWatcher::new(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    match event.kind {
                        EventKind::Create(_) | EventKind::Modify(_) => {
                            for path in event.paths {
                                if let Some(s) = path.to_str() {
                                    // Send path to processing queue
                                    // Using blocking send in async context is bad, but this is a sync callback.
                                    // We should use blocking_send or handle it.
                                    let _ = tx.blocking_send(s.to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Err(e) => eprintln!("Watch error: {:?}", e),
            }
        }, Config::default())?;

        watcher.watch(Path::new(&path), RecursiveMode::Recursive)?;

        Ok(Self { watcher })
    }
}
