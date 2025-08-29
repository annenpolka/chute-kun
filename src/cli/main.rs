use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{cursor, event, execute, terminal};
use ratatui::{backend::CrosstermBackend, Terminal};

use chute_kun::{app::App, config, ui};

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

    // Lightweight arg check for non-interactive helpers
    if let Some(arg1) = std::env::args().nth(1) {
        match arg1.as_str() {
            "--write-default-config" | "init-config" => {
                let (path, created) = config::write_default_config()?;
                if created {
                    println!("Wrote default config to {}", path.display());
                } else {
                    println!("Config already exists at {}", path.display());
                }
                return Ok(());
            }
            _ => {}
        }
    }

    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    // Real-time ticking (seconds) — accumulate elapsed millis and convert to seconds.
    let mut last_instant = Instant::now();
    let mut carry_millis: u64 = 0;

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        // Measure elapsed time and tick the app in whole seconds.
        let now = Instant::now();
        let elapsed = now.saturating_duration_since(last_instant);
        last_instant = now;
        carry_millis = carry_millis.saturating_add(elapsed.as_millis() as u64);
        while carry_millis >= 1000 {
            app.tick(1);
            carry_millis -= 1000;
        }
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
