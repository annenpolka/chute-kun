use std::io::stdout;
use std::time::Duration;

use anyhow::Result;
use crossterm::{cursor, event, execute, terminal};
use ratatui::{backend::CrosstermBackend, Terminal};

use chute_kun::{app::App, ui};

fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    execute!(terminal.backend_mut(), terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;
        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(k) = event::read()? {
                app.handle_key_event(k)
            }
        }
        if app.should_quit {
            break;
        }
    }

    restore_terminal(terminal)?;
    Ok(())
}
