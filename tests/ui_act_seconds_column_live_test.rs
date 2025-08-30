use chute_kun::{app::App, ui};
use crossterm::event::KeyCode;
use ratatui::{backend::TestBackend, Terminal};

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

// 第3カラムの Act がアクティブ/一時停止中に秒を表示する
#[test]
fn act_column_shows_seconds_for_active_task() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    app.handle_key(KeyCode::Enter); // start A
    app.tick(5); // 5s elapsed

    let clock = FixedClock(9 * 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    let buf = terminal.backend().buffer().clone();
    // 1行目の描画に "0m 5s" が含まれ、Actual(--:--) より左にある（= 第3カラム）
    let mut row1 = String::new();
    for x in 0..buf.area.width {
        row1.push_str(buf[(x, 3)].symbol());
    }
    let act_i = row1.find("0m 5s").expect("act seconds present");
    let actual_i = row1.find("--:--").expect("actual placeholder present");
    assert!(act_i < actual_i, "Act should be left of Actual: {}", row1);
}
