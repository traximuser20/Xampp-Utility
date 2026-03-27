use crate::app::{App, AppState};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub const MENU_ITEMS: [&str; 10] = [
    "[1] 🆕 Install Fresh XAMPP",
    "[2] ⚡ Upgrade/Update XAMPP",
    "[3] 🔄 Downgrade/Reinstall XAMPP",
    "[4] 💾 Backup Current Setup",
    "[5] 📂 Restore from Backup",
    "[6] 🗑️ Uninstall XAMPP",
    "[7] 🔍 Check for XAMPP Updates",
    "[8] 📍 Config XAMPP Path",
    "[9] 📂 Config Backup Dest",
    "[0] 🚪 Exit",
];

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Banner
            Constraint::Min(10),    // Content
            Constraint::Length(6),  // Logs
            Constraint::Length(3),  // Help/Status
        ])
        .split(f.area());

    draw_banner(f, chunks[0]);

    match app.state {
        AppState::MainMenu => draw_menu(f, chunks[1], app),
        _ => draw_task_view(f, chunks[1], app),
    }

    draw_logs(f, chunks[2], app);
    draw_help(f, chunks[3], app);
}

fn draw_banner(f: &mut Frame, area: Rect) {
    let banner = vec![
        Line::from(Span::styled(
            "⚡  __  __    _    ___  ___  ____   ____   ⚡",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            "🚀  \\ \\/ /   / \\   |  \\/  | |  _ \\ |  _ \\  🚀",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            "🔥   \\  /   / _ \\  | |\\/| | | ||_) | ||_)  🔥",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(Span::styled(
            "💎  /_/\\_\\ /_/ \\_\\ |_|  |_| |_|    |_|     💎",
            Style::default().fg(Color::Magenta),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "    ✨  _   _   ______   ___  ___    _____  _____  __   __  ✨",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "    📦 | | | | |__  __| |_ _| | |    |_ _| |__ __| \\_\\_/_/  📦",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "    🛠️ | |_| |   | |     |_|  | |__   | |    | |     | |    🛠️",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "    🌟 \\___/    |_|    |___| |____| |___|   |_|     |_|    🌟",
            Style::default().fg(Color::Cyan),
        )),
    ];

    let p = Paragraph::new(banner)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" XAMPP Utility Manager "),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

fn draw_menu(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(vec![Line::from(Span::styled(*item, style))])
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Main Menu "))
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol(">> ");

    f.render_widget(list, area);
}

fn draw_task_view(f: &mut Frame, area: Rect, app: &App) {
    let p = Paragraph::new(format!(
        "Current Task: {:?}\nStatus: {}\nProgress: {:.0}%",
        app.state,
        app.status,
        app.progress * 100.0
    ))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Task Progress "),
    )
    .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

fn draw_logs(f: &mut Frame, area: Rect, app: &App) {
    let logs: Vec<ListItem> = app
        .logs
        .iter()
        .rev()
        .take(5)
        .map(|log| ListItem::new(Line::from(Span::raw(log))))
        .collect();

    let list = List::new(logs).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Recent Logs "),
    );
    f.render_widget(list, area);
}

fn draw_help(f: &mut Frame, area: Rect, app: &App) {
    let msg = match app.state {
        AppState::MainMenu => "Arrows: Navigate | Enter: Select | Q: Quit",
        _ => "Press ESC to return to Main Menu",
    };
    let p = Paragraph::new(msg)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" Status: {} ", app.status)),
        )
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}
