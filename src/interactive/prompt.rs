use super::state::AppState;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
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
/// ```ignore
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
pub fn interactive_list(prompt: &str, items: &[&str]) -> anyhow::Result<Option<String>> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // enter raw mode and alternate screen
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    terminal.clear()?;

    let mut app = AppState::new(items);

    // Discard events crossterm buffers at launch on Windows (e.g. a spurious Enter that
    // would auto-select the first item). Best-effort: a read error must not abort.
    #[cfg(target_os = "windows")]
    {
        while poll(Duration::from_secs(0)).unwrap_or(false) {
            if read().is_err() {
                break;
            }
        }
    }

    let result = event_loop(&mut terminal, &mut app, prompt)?;

    // leave alternate screen and raw mode
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.clear()?;

    Ok(result)
}

fn draw_terminal<B>(
    terminal: &mut Terminal<B>,
    app: &mut AppState,
    prompt: &str,
) -> anyhow::Result<()>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(f.area());
        let items: Vec<ListItem> = app
            .items
            .displayed
            .iter()
            .map(|i| {
                let lines = vec![Line::from(*i)];
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
    })?;
    Ok(())
}

/// tries reading input from user
/// if succeeds, handles the input and returns Option<Option<String>> as return value
///
/// None: nothing was selected yet
/// Some(None): the user selected nothing (quit with ESC)
/// Some(Some(res)): the user selected an item
fn read_keys(app: &mut AppState) -> anyhow::Result<Option<Option<String>>> {
    if poll(Duration::from_millis(POLL_RATE))? {
        if let Event::Key(x) = read()? {
            return handle_key_event(app, x);
        }
    }
    Ok(None)
}

/// handles a single key event, mutating `app` and returning the selection result.
///
/// See `read_keys` for the meaning of the `Option<Option<String>>` return value.
fn handle_key_event(app: &mut AppState, x: KeyEvent) -> anyhow::Result<Option<Option<String>>> {
    // Accept presses, and auto-repeat for held nav/backspace keys. Ignore other events
    // (e.g. Windows Release) to avoid spurious input.
    let repeat_nav = x.kind == KeyEventKind::Repeat
        && matches!(
            x.code,
            KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right | KeyCode::Backspace
        );
    if !(x.is_press() || repeat_nav) {
        return Ok(None);
    }

    // CTRL-C is the usual stop command
    // which is disabled by default because of raw mode
    if x.code == KeyCode::Char('c') && x.modifiers.contains(KeyModifiers::CONTROL) {
        // restore the terminal before exiting
        let _ = stdout().execute(LeaveAlternateScreen);
        disable_raw_mode().ok();
        std::process::exit(0);
    }
    let selection = match x.code {
        KeyCode::Esc => Some(None),
        KeyCode::Up | KeyCode::Left => {
            app.items.previous();
            None
        }
        KeyCode::Down | KeyCode::Right => {
            app.items.next();
            None
        }
        // if no selection, None
        // else Some(Some(selection))
        KeyCode::Enter => app.get_selected().map(Some),
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
    Ok(selection)
}

fn event_loop<B>(
    terminal: &mut Terminal<B>,
    app: &mut AppState<'_>,
    prompt: &str,
) -> anyhow::Result<Option<String>>
where
    B: Backend,
    B::Error: std::error::Error + Send + Sync + 'static,
{
    loop {
        draw_terminal(terminal, app, prompt)?;

        match read_keys(app)? {
            Some(Some(res)) => {
                println!();
                return Ok(Some(res));
            }
            Some(None) => {
                // user interrupted selection
                return Ok(None);
            }
            None => {
                // nothing was selected, continue
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(code: KeyCode, kind: KeyEventKind) -> KeyEvent {
        KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind)
    }

    fn enter(kind: KeyEventKind) -> KeyEvent {
        key(KeyCode::Enter, kind)
    }

    #[test]
    fn enter_release_is_ignored() {
        let items = &["eka", "toka"];
        let mut app = AppState::new(items);

        // A stray Enter release must not count as a selection.
        let result = handle_key_event(&mut app, enter(KeyEventKind::Release)).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn enter_repeat_is_ignored() {
        let items = &["eka", "toka"];
        let mut app = AppState::new(items);

        let result = handle_key_event(&mut app, enter(KeyEventKind::Repeat)).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn enter_press_selects_current_item() {
        let items = &["eka", "toka"];
        let mut app = AppState::new(items);

        // index 0 is pre-selected by `AppState::new`
        let result = handle_key_event(&mut app, enter(KeyEventKind::Press)).unwrap();
        assert_eq!(result, Some(Some(String::from("eka"))));
    }

    #[test]
    fn arrow_release_does_not_move_selection() {
        let items = &["eka", "toka", "kolmas"];
        let mut app = AppState::new(items);

        // A stray Down release must not advance the cursor (Windows emits Press+Release per tap).
        handle_key_event(&mut app, key(KeyCode::Down, KeyEventKind::Release)).unwrap();
        assert_eq!(app.get_selected().unwrap(), "eka");

        handle_key_event(&mut app, key(KeyCode::Up, KeyEventKind::Release)).unwrap();
        assert_eq!(app.get_selected().unwrap(), "eka");
    }

    #[test]
    fn down_press_moves_exactly_one_row() {
        let items = &["eka", "toka", "kolmas"];
        let mut app = AppState::new(items);

        // One tap = one Press = one move.
        handle_key_event(&mut app, key(KeyCode::Down, KeyEventKind::Press)).unwrap();
        assert_eq!(app.get_selected().unwrap(), "toka");

        handle_key_event(&mut app, key(KeyCode::Down, KeyEventKind::Press)).unwrap();
        assert_eq!(app.get_selected().unwrap(), "kolmas");
    }

    #[test]
    fn down_repeat_moves_selection() {
        let items = &["eka", "toka", "kolmas"];
        let mut app = AppState::new(items);

        // Holding Down auto-repeats: each Repeat advances the cursor like a Press.
        handle_key_event(&mut app, key(KeyCode::Down, KeyEventKind::Repeat)).unwrap();
        assert_eq!(app.get_selected().unwrap(), "toka");
    }
}
