use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, Terminal};

// カレンダービュー: 1分未満（start==end）の実績は表示しない（▓が描かれない）
#[test]
fn subminute_actual_sessions_are_hidden() {
    let backend = TestBackend::new(72, 12);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();

    // 1タスク、0分セッション（開始と終了の分が同じ）
    let base = app.config.day_start_minutes;
    app.add_task("Tiny", 30);
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: base + 15, end_min: Some(base + 15) });
    }

    app.toggle_display_mode(); // Calendar
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // 画面全体から '▓' が消えていること（Actualブロックが描かれない）
    for y in 0..buf.area.height {
        for x in 0..buf.area.width {
            assert_ne!(buf[(x, y)].symbol(), "▓", "should hide sub-minute blocks");
        }
    }
}

