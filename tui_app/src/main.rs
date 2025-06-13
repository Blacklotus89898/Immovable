use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use walkdir::WalkDir;

use std::io::{self, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let all_files: Vec<String> = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().display().to_string())
        .collect();

    let matcher = SkimMatcherV2::default();

    let mut input = String::new();
    let mut matched_files = all_files.clone();

    let mut selected_idx: usize = 0;
    let mut list_state = ListState::default();
    list_state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(1)])
                .split(f.size());

            let input_widget = Paragraph::new(Text::from(input.clone()))
                .block(Block::default().borders(Borders::ALL).title("Search"));
            f.render_widget(input_widget, chunks[0]);

            let items: Vec<ListItem> = matched_files
                .iter()
                .take(20)
                .map(|i| ListItem::new(i.clone()))
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("Results"))
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::Black));
            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        if event::poll(std::time::Duration::from_millis(10))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == event::KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            input.push(c);
                            selected_idx = 0;
                        }

                        KeyCode::Backspace => {
                            input.pop();
                            selected_idx = 0;
                        }
                        KeyCode::Esc => break,
                        KeyCode::Up => {
                            if selected_idx > 0 {
                                selected_idx -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if selected_idx + 1 < matched_files.len().min(20) {
                                selected_idx += 1;
                            }
                        }
                        _ => {}
                    }

                    matched_files = if input.is_empty() {
                        all_files.clone()
                    } else {
                        let mut scored: Vec<(String, i64)> = all_files
                            .iter()
                            .filter_map(|f| matcher.fuzzy_match(f, &input).map(|s| (f.clone(), s)))
                            .collect();
                        scored.sort_by(|a, b| b.1.cmp(&a.1));
                        scored.into_iter().map(|(f, _)| f).collect()
                    };

                    list_state.select(if matched_files.is_empty() {
                        None
                    } else {
                        Some(selected_idx.min(matched_files.len().min(20) - 1))
                    });
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
