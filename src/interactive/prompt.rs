use super::state::AppState;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Spans,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

use std::{io::stdout, time::Duration};

/// control the maximum waiting time for event availability
/// in this case, the value should not really matter,
/// as the content does not update while waiting for events
///
/// see https://docs.rs/crossterm/0.14.0/crossterm/event/fn.poll.html
const POLL_RATE: u64 = 1000;

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

    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).unwrap();

    let result = event_loop(terminal, items, prompt);

    disable_raw_mode().unwrap();

    if let Some(result) = result {
        Some(result)
    } else {
        None
    }
}

fn event_loop<B>(mut terminal: Terminal<B>, items: Vec<String>, prompt: &str) -> Option<String>
where
    B: Backend,
{
    terminal.clear().unwrap();
    //let items = items
    //.iter()
    //.zip(0..)
    //.map(|(a, b)| (a.to_owned(), b))
    //.collect::<Vec<_>>();

    let mut app = AppState::new(items);

    let mut result = None;
    // set the highlighted item to be the first in the list
    app.items.next();
    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(f.size());
                let items: Vec<ListItem> = app
                    .items
                    .displayed
                    .iter()
                    .map(|i| {
                        let lines = vec![Spans::from(i.clone())];
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

        if poll(Duration::from_millis(POLL_RATE)).unwrap() {
            if let Ok(Event::Key(x)) = read() {
                // CTRL-C is the usual stop command
                // which is disabled by default because of raw mode
                if x.code == KeyCode::Char('c') && x.modifiers == KeyModifiers::CONTROL {
                    break;
                }
                match x.code {
                    KeyCode::Esc => break,
                    KeyCode::Up | KeyCode::Left => app.items.previous(),
                    KeyCode::Down | KeyCode::Right => app.items.next(),
                    KeyCode::Enter => {
                        result = app.get_selected();
                        break;
                    }
                    KeyCode::Char(c) => {
                        app.push_filter(c);
                    }
                    KeyCode::Backspace => app.pop_filter(),
                    _ => {}
                }
            }
        }
    }
    terminal.clear().unwrap();

    result
}
