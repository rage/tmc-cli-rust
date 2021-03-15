use super::state::AppState;
use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use std::{io::stdout, time::Duration};

/// display an interactive prompt to ask the user to select an item
///
/// example:
/// ```
/// let prompt = "Choose your organization:";
/// let items = vec![String::from("Eka"), String::from("Toka"), String::from("Kolmas"),
/// String::from("Neljäs")];
///
/// let choice = interactive_list(prompt, items);
///
/// if let Some(choice) = choice {
///     println!("You chose: {}", choice);
/// }
/// ```
pub fn interactive_list(prompt: &str, items: Vec<String>) -> Option<String> {
    enable_raw_mode().unwrap();

    let items = items
        .iter()
        .zip(0..)
        .map(|(a, b)| (a.to_owned(), b))
        .collect::<Vec<_>>();
    let mut result = 0;
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.clear().unwrap();

    let mut app = AppState::new(items.clone());
    app.items.next();
    let poll_rate = 10;
    // todo filtering
    //let mut filter_word = String::from("");
    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());
                let items: Vec<ListItem> = app
                    .items
                    .items
                    .iter()
                    // todo filtering
                    //.filter(|i| i.0.contains(&filter_word))
                    .map(|i| {
                        let lines = vec![Spans::from(i.clone().0)];
                        ListItem::new(lines).style(Style::default())
                    })
                    .collect();
                let items = List::new(items)
                    .block(Block::default().borders(Borders::NONE).title(prompt))
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> ");
                f.render_stateful_widget(items, chunks[0], &mut app.items.state);
            })
            .unwrap();

        if poll(Duration::from_millis(poll_rate)).unwrap() {
            if let Ok(Event::Key(x)) = read() {
                match x.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Left => app.items.previous(),
                    KeyCode::Down | KeyCode::Right => app.items.next(),
                    KeyCode::Enter => {
                        result = app.items.get_current().unwrap_or_default();
                        break;
                    }
                    //KeyCode::Char(_c) => {
                    // todo filtering
                    // filter_word.push(c);
                    //}
                    //KeyCode::Backspace => {
                    // todo filtering
                    // filter_word.pop();
                    //}
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode().unwrap();

    terminal.clear().unwrap();

    Some(items[result].0.clone())
}
