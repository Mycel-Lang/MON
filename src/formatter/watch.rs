// Watch mode implementation for continuous formatting

use colored::Colorize;
use miette::Result;
use notify::{Event, EventKind, RecursiveMode, Watcher, event::ModifyKind};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

pub struct WatchMode {
    paths: Vec<PathBuf>,
    write: bool,
}

impl WatchMode {
    pub fn new(paths: Vec<PathBuf>, write: bool) -> Self {
        Self { paths, write }
    }
    pub fn run(&self) -> Result<()> {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })
        .map_err(|e| miette::miette!("Failed to create watcher: {}", e))?;

        // Watch all provided paths
        for path in &self.paths {
            if path.is_file() {
                watcher
                    .watch(path, RecursiveMode::NonRecursive)
                    .map_err(|e| miette::miette!("Failed to watch {}: {}", path.display(), e))?;
                println!("{} {}", "Watching".green().bold(), path.display());
            } else if path.is_dir() {
                watcher
                    .watch(path, RecursiveMode::Recursive)
                    .map_err(|e| miette::miette!("Failed to watch {}: {}", path.display(), e))?;
                println!("{} {} (recursively)", "Watching".green().bold(), path.display());
            }
        }

        println!("{}", "Press Ctrl+C to stop".to_string().dimmed());

        // Debounce map to avoid formatting the same file multiple times in quick succession
        let mut last_format: std::collections::HashMap<PathBuf, std::time::Instant> =
            std::collections::HashMap::new();
        let debounce_duration = Duration::from_millis(100);

        loop {
            match rx.recv() {
                Ok(event) => {
                    if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                        for path in event.paths {
                            if !path.extension().is_some_and(|e| e == "mon") {
                                continue;
                            }

                            // Debounce
                            let now = std::time::Instant::now();
                            if let Some(last) = last_format.get(&path)
                                && now.duration_since(*last) < debounce_duration {
                                    continue;
                                }
                            last_format.insert(path.clone(), now);

                            // Format the file
                            if let Err(e) = self.format_file(&path) {
                                eprintln!(
                                    "{} formatting {}: {}",
                                    "Error".red().bold(),
                                    path.display(),
                                    e
                                );
                            } else {
                                println!("{} {}", "Formatted".green().bold(), path.display());
                            }
                        }
                    }
                }
                Err(e) => {
                    return Err(miette::miette!("Watch error: {}", e));
                }
            }
        }
    }

    fn format_file(&self, path: &Path) -> Result<()> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| miette::miette!("Failed to read {}: {}", path.display(), e))?;

        let formatter = crate::formatter::Formatter::new(crate::formatter::FormatConfig::default());
        let formatted = formatter.format(&source)?;

        if self.write {
            std::fs::write(path, formatted)
                .map_err(|e| miette::miette!("Failed to write {}: {}", path.display(), e))?;
        } else {
            println!("{}", formatted);
        }

        Ok(())
    }
}
