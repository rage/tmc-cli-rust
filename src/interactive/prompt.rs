use super::state::ListView;
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
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
pub fn interactive_list(
    prompt: &str,
    items: &[&str],
    help: Option<&str>,
) -> anyhow::Result<Option<String>> {
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    enable_raw_mode()?;
    let mut app = ListView::new(items, help);
    let result = event_loop(&mut terminal, &mut app, prompt)?;

    disable_raw_mode()?;

    terminal.clear()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(result)
}

fn draw_terminal<B>(
    terminal: &mut Terminal<B>,
    app: &mut ListView,
    prompt: &str,
) -> anyhow::Result<()>
where
    B: Backend,
{
    terminal.draw(|f| {
        let vertical_layout = if app.help.is_some() {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_slice())
                .split(f.size())
        } else {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_slice())
                .split(f.size())
        };
        let top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_slice())
            .split(vertical_layout[0]);

        let items: Vec<ListItem> = app
            .items
            .displayed
            .iter()
            .map(|i| ListItem::new(*i).style(Style::default()))
            .collect();
        let items = List::new(items)
            .block(Block::default().borders(Borders::NONE).title(prompt))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(items, top_layout[0], &mut app.items.state);

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
        f.render_widget(text, top_layout[1]);
        if let Some(help) = app.help {
            let text = Paragraph::new(Span::raw(help)).wrap(Wrap { trim: false });
            f.render_widget(text, vertical_layout[1]);
        }
    })?;
    Ok(())
}

/// tries reading input from user
/// if succeeds, handles the input and returns Option<Option<String>> as return value
///
/// None: nothing was selected yet
/// Some(None): the user selected nothing (quit with ESC)
/// Some(Some(res)): the user selected an item
fn read_keys(app: &mut ListView) -> anyhow::Result<Option<Option<String>>> {
    if poll(Duration::from_millis(POLL_RATE))? {
        if let Event::Key(x) = read()? {
            // CTRL-C is the usual stop command
            // which is disabled by default because of raw mode
            if x.code == KeyCode::Char('c') && x.modifiers == KeyModifiers::CONTROL {
                // respect the interrupt and exit immediately
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
            return Ok(selection);
        }
    }
    Ok(None)
}

fn event_loop<B>(
    terminal: &mut Terminal<B>,
    app: &mut ListView<'_>,
    prompt: &str,
) -> anyhow::Result<Option<String>>
where
    B: Backend,
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
