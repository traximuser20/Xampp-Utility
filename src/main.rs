use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};

mod app;
mod config;
mod tasks;
mod ui;

use crate::app::{App, AppState};

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let mut app = App::new()?;
    let res = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

use tokio::sync::mpsc;

#[derive(Debug)]
pub enum AppEvent {
    Log(String),
    Status(String),
    Progress(f32),
    Done(AppState),
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: std::error::Error + Send + Sync + 'static,
{
    let (tx, mut rx) = mpsc::channel(100);

    loop {
        // Handle background events
        while let Ok(event) = rx.try_recv() {
            match event {
                AppEvent::Log(msg) => app.log(msg),
                AppEvent::Status(status) => app.status = status,
                AppEvent::Progress(p) => app.progress = p,
                AppEvent::Done(state) => {
                    app.state = state;
                    app.log("Task completed.".to_string());
                }
            }
        }

        terminal.draw(|f| ui::ui(f, app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match app.state {
                        AppState::MainMenu => match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.next_menu_item(ui::MENU_ITEMS.len())
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.previous_menu_item(ui::MENU_ITEMS.len())
                            }
                            KeyCode::Enter => {
                                match app.menu_index {
                                    0 => {
                                        app.state = AppState::Installing;
                                        start_install(app, tx.clone());
                                    }
                                    3 => {
                                        app.state = AppState::BackingUp;
                                        start_backup(app, tx.clone());
                                    }
                                    9 => app.should_quit = true,
                                    _ => app.state = AppState::CheckingUpdates, // Placeholder for others
                                }
                            }
                            _ => {}
                        },
                        _ => {
                            if key.code == KeyCode::Esc {
                                app.state = AppState::MainMenu;
                            }
                        }
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn start_backup(app: &App, tx: mpsc::Sender<AppEvent>) {
    let xampp_path = app.config.xampp_path.clone();
    let backup_path = app.config.backup_path.clone();

    tokio::spawn(async move {
        let _ = tx.send(AppEvent::Status("Backing up...".to_string())).await;
        let _ = tx.send(AppEvent::Log(format!("Starting backup of {:?}...", xampp_path))).await;
        
        match crate::tasks::backup::perform_backup(&xampp_path, &backup_path) {
            Ok(path) => {
                let _ = tx.send(AppEvent::Log(format!("Backup successful: {:?}", path))).await;
            }
            Err(e) => {
                let _ = tx.send(AppEvent::Log(format!("Backup failed: {}", e))).await;
            }
        }
        let _ = tx.send(AppEvent::Done(AppState::MainMenu)).await;
    });
}

fn start_install(app: &App, tx: mpsc::Sender<AppEvent>) {
    let target_dir = app.config.xampp_path.clone();
    let releases = crate::tasks::install::get_xampp_releases();
    let latest_url = releases.values().next().cloned().unwrap_or_default();

    tokio::spawn(async move {
        let _ = tx.send(AppEvent::Status("Downloading XAMPP...".to_string())).await;
        let temp_zip = std::env::temp_dir().join("xampp_latest.zip");
        
        match crate::tasks::install::download_xampp(&latest_url, &temp_zip).await {
            Ok(_) => {
                let _ = tx.send(AppEvent::Log("Download complete. Extracting...".to_string())).await;
                let _ = tx.send(AppEvent::Status("Extracting...".to_string())).await;
                match crate::tasks::install::install_xampp(&temp_zip, &target_dir) {
                    Ok(_) => {
                        let _ = tx.send(AppEvent::Log("Installation successful!".to_string())).await;
                    }
                    Err(e) => {
                        let _ = tx.send(AppEvent::Log(format!("Installation failed: {}", e))).await;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(AppEvent::Log(format!("Download failed: {}", e))).await;
            }
        }
        let _ = tx.send(AppEvent::Done(AppState::MainMenu)).await;
    });
}
