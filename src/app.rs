use crate::config::Config;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    MainMenu,
    Installing,
    BackingUp,
    Restoring,
    CheckingUpdates,
    ConfigXamppPath,
    ConfigBackupPath,
    Exiting,
}

pub struct App {
    pub config: Config,
    pub state: AppState,
    pub menu_index: usize,
    pub logs: Vec<String>,
    pub should_quit: bool,
    pub progress: f32,
    pub status: String,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut config = Config::load()?;
        
        // Run discovery if xampp_path doesn't exist
        if !config.xampp_path.exists() {
            if let Some(path) = crate::tasks::discovery::discover_xampp() {
                config.xampp_path = path;
                let _ = config.save();
            }
        }

        Ok(Self {
            config,
            state: AppState::MainMenu,
            menu_index: 0,
            logs: Vec::new(),
            should_quit: false,
            progress: 0.0,
            status: String::from("Ready"),
        })
    }

    pub fn log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    pub fn next_menu_item(&mut self, count: usize) {
        self.menu_index = (self.menu_index + 1) % count;
    }

    pub fn previous_menu_item(&mut self, count: usize) {
        if self.menu_index > 0 {
            self.menu_index -= 1;
        } else {
            self.menu_index = count - 1;
        }
    }
}
