use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{cursor, event, execute, terminal};
use crossterm::event::{KeyboardEnhancementFlags, PushKeyboardEnhancementFlags, PopKeyboardEnhancementFlags};
use ratatui::{backend::CrosstermBackend, Terminal};

use chute_kun::{app::App, ui};
use chute_kun::config::Config;
use chute_kun::storage;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    terminal::enable_raw_mode()?;
    // Enter alternate screen + hide cursor first
    execute!(stdout(), terminal::EnterAlternateScreen, cursor::Hide)?;
    // Try to enable progressive keyboard enhancement so modifiers like Shift+Enter are reported.
    // NOTE: Avoid REPORT_ALL_KEYS_AS_ESCAPE_CODES because crossterm cannot yet decode
    // Unicode code points for CSI-u mode, which breaks IME (e.g., Japanese) input.
    // Keeping DISAMBIGUATE_ESCAPE_CODES + REPORT_EVENT_TYPES is enough for most
    // modifier reporting without regressing non-ASCII text input.
    if terminal::supports_keyboard_enhancement().unwrap_or(false) {
        let flags = KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
            | KeyboardEnhancementFlags::REPORT_EVENT_TYPES;
        let _ = execute!(stdout(), PushKeyboardEnhancementFlags(flags));
    }
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // Best-effort pop keyboard enhancement flags if they were pushed
    let _ = execute!(terminal.backend_mut(), PopKeyboardEnhancementFlags);
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

    // Simple flag handling to initialize config and exit.
    if std::env::args().any(|a| a == "--init-config") {
        let path = Config::write_default_file()?;
        println!("wrote config to {}", path.display());
        return Ok(());
    }

    // Optional state path override: --state <path>
    let mut args = std::env::args().skip(1);
    let mut state_path_override: Option<std::path::PathBuf> = None;
    while let Some(a) = args.next() {
        if a == "--state" {
            if let Some(p) = args.next() {
                state_path_override = Some(std::path::PathBuf::from(p));
            } else {
                eprintln!("--state requires a file path");
                std::process::exit(2);
            }
        }
    }

    let mut terminal = setup_terminal()?;

    // Load config and state snapshot (if found) from XDG data path or override.
    let cfg = Config::load();
    let chosen_path = state_path_override
        .or_else(|| storage::default_state_path())
        .expect("could not resolve default state path");

    let mut app = match storage::load_from_path(&chosen_path, cfg.clone())? {
        Some(a) => a,
        None => App::with_config(cfg),
    };

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
            match event::read()? {
                event::Event::Key(k) => app.handle_key_event(k),
                event::Event::Paste(s) => app.handle_paste(&s),
                _ => {}
            }
        }
        if app.should_quit {
            break;
        }
    }

    // Auto-save snapshot on exit.
    if let Err(e) = storage::save_to_path(&app, &chosen_path) {
        // Avoid crashing; report to stderr after restoring terminal.
        tracing::error!("failed to save snapshot: {e}");
    }
    restore_terminal(terminal)?;
    Ok(())
}
