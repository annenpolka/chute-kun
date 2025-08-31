use chute_kun::{app::App, task::Session, ui};
use ratatui::{backend::TestBackend, Terminal};

// 同一タスクの重複（視覚的に同一行にかかるセッション）は1列にまとめて表示する
#[test]
fn same_task_overlapping_sessions_merge_into_one_column() {
    let backend = TestBackend::new(72, 14);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new();
    let base = app.config.day_start_minutes;
    app.add_task("Alpha", 90);
    // 故意に同一タスクで重なる2セッションを作成（テスト用）
    if let Some(t) = app.day.tasks.get_mut(0) {
        t.sessions.push(Session { start_min: base + 10, end_min: Some(base + 35) });
        t.sessions.push(Session { start_min: base + 20, end_min: Some(base + 50) });
    }

    app.toggle_display_mode();
    // 描画（時刻は十分後ろ）
    terminal.draw(|f| ui::draw(f, &app)).unwrap();
    let buf = terminal.backend().buffer().clone();

    // Actual レーン範囲だけを対象に 'A' が1以下であることを確認
    let area = ratatui::layout::Rect { x: 0, y: 0, width: buf.area.width, height: buf.area.height };
    let (_tabs, _banner, list, _help) = ui::compute_layout(&app, area);
    let gutter = 6u16; let lanes_w = list.width.saturating_sub(gutter + 2);
    let lane_w = (lanes_w.saturating_sub(1)) / 2; let plan_x0 = list.x + gutter + 1; let act_x0 = plan_x0 + lane_w + 1;
    for y in list.y..list.y + list.height {
        let mut s = String::new();
        for x in act_x0..act_x0 + lane_w { s.push_str(buf[(x, y)].symbol()); }
        let count_a = s.matches('A').count();
        assert!(count_a <= 1, "row {} has multiple 'A' in act lane: {}", y, s);
    }
}
