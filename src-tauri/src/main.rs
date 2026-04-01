// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Emitter};
use crate::config::Config;
use serde::Serialize;

mod config;
mod tasks;

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
    progress: f32,
    status: String,
}

#[tauri::command]
fn get_config() -> Config {
    Config::load().unwrap_or_default()
}

#[tauri::command]
async fn start_backup(app: AppHandle) -> Result<(), String> {
    let config = Config::load().unwrap_or_default();
    let xampp_path = config.xampp_path.clone();
    let backup_path = config.backup_path.clone();

    tokio::task::spawn_blocking(move || {
        let app_clone = app.clone();
        let _ = app.emit("log", Payload {
            message: "Starting backup...".to_string(),
            progress: 0.0,
            status: "Backing up...".to_string(),
        });

        match crate::tasks::backup::perform_backup(&xampp_path, &backup_path, move |p| {
            let _ = app_clone.emit("progress", Payload {
                message: format!("Backup progress: {:.0}%", p * 100.0),
                progress: p,
                status: "Backing up...".to_string(),
            });
        }) {
            Ok(path) => {
                let _ = app.emit("log", Payload {
                    message: format!("Backup successful: {:?}", path),
                    progress: 1.0,
                    status: "Ready".to_string(),
                });
            }
            Err(e) => {
                let _ = app.emit("log", Payload {
                    message: format!("Backup failed: {}", e),
                    progress: 0.0,
                    status: "Error".to_string(),
                });
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn start_restore(app: AppHandle) -> Result<(), String> {
    let config = Config::load().unwrap_or_default();
    let xampp_path = config.xampp_path.clone();
    let backup_dest = config.backup_path.clone();

    tokio::task::spawn_blocking(move || {
        let app_clone = app.clone();
        
        // Find latest zip
        let entries = match std::fs::read_dir(&backup_dest) {
            Ok(e) => e,
            Err(_) => {
                let _ = app.emit("log", Payload {
                    message: "Could not read backup directory.".to_string(),
                    progress: 0.0,
                    status: "Error".to_string(),
                });
                return;
            }
        };

        let mut zips: Vec<_> = entries.flatten()
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "zip"))
            .collect();
        
        zips.sort_by_key(|e| std::cmp::Reverse(e.metadata().and_then(|m| m.created()).ok()));

        if let Some(latest) = zips.first() {
            let backup_path = latest.path();
            let _ = app.emit("log", Payload {
                message: format!("Restoring from: {:?}", backup_path.file_name()),
                progress: 0.0,
                status: "Restoring...".to_string(),
            });
            
            match crate::tasks::restore::perform_restore(&backup_path, &xampp_path, move |p| {
                let _ = app_clone.emit("progress", Payload {
                    message: format!("Restore progress: {:.0}%", p * 100.0),
                    progress: p,
                    status: "Restoring...".to_string(),
                });
            }) {
                Ok(_) => {
                    let _ = app.emit("log", Payload {
                        message: "Restore successful!".to_string(),
                        progress: 1.0,
                        status: "Ready".to_string(),
                    });
                }
                Err(e) => {
                    let _ = app.emit("log", Payload {
                        message: format!("Restore failed: {}", e),
                        progress: 0.0,
                        status: "Error".to_string(),
                    });
                }
            }
        } else {
            let _ = app.emit("log", Payload {
                message: "No backup files found.".to_string(),
                progress: 0.0,
                status: "Ready".to_string(),
            });
        }
    });

    Ok(())
}

#[tauri::command]
async fn start_install(app: AppHandle) -> Result<(), String> {
    let config = Config::load().unwrap_or_default();
    let target_dir = config.xampp_path.clone();
    let releases = crate::tasks::install::get_xampp_releases();
    let latest_url = releases.values().next().cloned().unwrap_or_default();

    tokio::spawn(async move {
        let app_clone = app.clone();
        let _ = app.emit("log", Payload {
            message: "Starting download...".to_string(),
            progress: 0.0,
            status: "Downloading...".to_string(),
        });

        let temp_zip = std::env::temp_dir().join("xampp_latest.zip");
        
        match crate::tasks::install::download_xampp(&latest_url, &temp_zip, move |p| {
            let _ = app_clone.emit("progress", Payload {
                message: format!("Download progress: {:.0}%", p * 100.0),
                progress: p,
                status: "Downloading...".to_string(),
            });
        }).await {
            Ok(_) => {
                let app_clone2 = app.clone();
                let _ = app.emit("log", Payload {
                    message: "Download complete. Extracting...".to_string(),
                    progress: 0.0,
                    status: "Extracting...".to_string(),
                });
                
                tokio::task::spawn_blocking(move || {
                    let app_clone3 = app_clone2.clone();
                    match crate::tasks::install::install_xampp(&temp_zip, &target_dir, move |p| {
                        let _ = app_clone3.emit("progress", Payload {
                            message: format!("Extraction progress: {:.0}%", p * 100.0),
                            progress: p,
                            status: "Extracting...".to_string(),
                        });
                    }) {
                        Ok(_) => {
                            let _ = app_clone2.emit("log", Payload {
                                message: "Installation successful!".to_string(),
                                progress: 1.0,
                                status: "Ready".to_string(),
                            });
                        }
                        Err(e) => {
                            let _ = app_clone2.emit("log", Payload {
                                message: format!("Installation failed: {}", e),
                                progress: 0.0,
                                status: "Error".to_string(),
                            });
                        }
                    }
                });
            }
            Err(e) => {
                let _ = app.emit("log", Payload {
                    message: format!("Download failed: {}", e),
                    progress: 0.0,
                    status: "Error".to_string(),
                });
            }
        }
    });

    Ok(())
}

#[tauri::command]
async fn check_updates(app: AppHandle) -> Result<(), String> {
    let config = Config::load().unwrap_or_default();
    let current_version = crate::tasks::discovery::get_xampp_version(&config.xampp_path);
    let releases = crate::tasks::install::get_xampp_releases();

    tokio::spawn(async move {
        let _ = app.emit("log", Payload {
            message: "Checking for updates...".to_string(),
            progress: 0.5,
            status: "Checking...".to_string(),
        });

        if let Some(latest_version) = releases.keys().last() {
             let msg = match current_version {
                Some(current) => {
                    if current == *latest_version {
                        "You are up to date! 🎉".to_string()
                    } else {
                        format!("Update available! {} -> {}", current, latest_version)
                    }
                }
                None => format!("Latest version: v{}", latest_version),
            };
            let _ = app.emit("log", Payload {
                message: msg,
                progress: 1.0,
                status: "Ready".to_string(),
            });
        } else {
            let _ = app.emit("log", Payload {
                message: "Failed to fetch releases.".to_string(),
                progress: 0.0,
                status: "Ready".to_string(),
            });
        }
    });

    Ok(())
}

#[tauri::command]
fn discover_xampp() -> Option<String> {
    crate::tasks::discovery::discover_xampp().map(|p| p.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            start_backup,
            start_restore,
            start_install,
            check_updates,
            discover_xampp
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
