use super::state::AppState;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Wrap,
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

fn draw_terminal<B>(terminal: &mut Terminal<B>, app: &mut AppState, prompt: &str)
where
    B: Backend,
{
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
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

            // if the user hasn't written anything yet, display the help message in its place
            let text = if app.filter.is_empty() {
                Paragraph::new(Span::styled(
                    "Press keys to filter",
                    Style::default().add_modifier(Modifier::ITALIC),
                ))
                .wrap(Wrap { trim: true })
            } else {
                Paragraph::new(Span::raw(app.filter.clone())).wrap(Wrap { trim: true })
            };
            f.render_widget(text, chunks[1]);
        })
        .unwrap();
}

/// tries reading input from user
/// if succeeds, handles the input and returns Option<Option<String>> as return value
///
/// None: nothing was selected yet
/// Some(None): the user interrupted and the process should quit
/// Some(Some(res)): the user has selected an item
fn read_keys(app: &mut AppState) -> Option<Option<String>> {
    if poll(Duration::from_millis(POLL_RATE)).unwrap() {
        if let Ok(Event::Key(x)) = read() {
            // CTRL-C is the usual stop command
            // which is disabled by default because of raw mode
            if x.code == KeyCode::Char('c') && x.modifiers == KeyModifiers::CONTROL {
                return Some(None);
            }
            return match x.code {
                KeyCode::Esc => Some(None),
                KeyCode::Up | KeyCode::Left => {
                    app.items.previous();
                    None
                }
                KeyCode::Down | KeyCode::Right => {
                    app.items.next();
                    None
                }
                KeyCode::Enter => Some(app.get_selected()),
                KeyCode::Char(c) => {
                    app.push_filter(c);
                    None
                }
                KeyCode::Backspace => {
                    app.pop_filter();
                    None
                }
                _ => None,
            };
        }
    }

    None
}

fn event_loop<B>(mut terminal: Terminal<B>, items: Vec<String>, prompt: &str) -> Option<String>
where
    B: Backend,
{
    terminal.clear().unwrap();

    let mut app = AppState::new(items);

    let mut result = None;
    loop {
        draw_terminal(&mut terminal, &mut app, prompt);

        if let Some(res) = read_keys(&mut app) {
            if res.is_some() {
                result = res;
            }
            break;
        }
    }
    terminal.clear().unwrap();

    println!();

    result
}
