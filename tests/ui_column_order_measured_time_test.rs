use chute_kun::{app::App, ui};
use ratatui::{backend::TestBackend, Terminal};

struct FixedClock(u16);
impl chute_kun::clock::Clock for FixedClock {
    fn now_minutes(&self) -> u16 {
        self.0
    }
}

// 計測時間（act: Xm Ys の累計）を左から三番目に配置する
#[test]
fn measured_time_is_third_column_from_left() {
    let backend = TestBackend::new(80, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut app = App::new();
    app.add_task("A", 30);
    app.add_task("B", 20);

    let clock = FixedClock(9 * 60);
    terminal.draw(|f| ui::draw_with_clock(f, &app, &clock)).unwrap();

    let buf = terminal.backend().buffer().clone();

    // ヘッダー行（y=2）を取得して、Plan -> Task -> Act -> Actual の順序になっていることを確認
    let mut header_line = String::new();
    for x in 0..buf.area.width {
        header_line.push_str(buf[(x, 2)].symbol());
    }
    let p = header_line.find("Plan").expect("header has Plan");
    let t = header_line.find("Task").expect("header has Task");
    let a1 = header_line.find("Act").expect("header has Act");
    let a2 = header_line.find("Actual").expect("header has Actual");
    assert!(p < t && t < a1 && a1 < a2, "header order wrong: {}", header_line);

    // 1行目のデータ行（y=3）も Plan(09:00) -> Task(A) -> Act(0m 0s) -> Actual(--:--) の順で並ぶことを軽く確認
    let mut row1 = String::new();
    for x in 0..buf.area.width {
        row1.push_str(buf[(x, 3)].symbol());
    }
    let p_i = row1.find("09:00").expect("row has planned time");
    let t_i = row1.find(" A").unwrap_or_else(|| row1.find("A").unwrap());
    let act_i = row1.find("0m 0s").expect("row has act seconds");
    let actual_i = row1.find("--:--").expect("row has actual placeholder");
    assert!(p_i < t_i && t_i < act_i && act_i < actual_i, "row order wrong: {}", row1);
}
