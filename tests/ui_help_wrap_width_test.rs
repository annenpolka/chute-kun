use chute_kun::{app::App, ui::help_lines_for_width};
use unicode_width::UnicodeWidthStr;

// Red -> Green: help should wrap to the given width without overflowing
#[test]
fn help_wraps_to_width_without_overflow() {
    let app = App::new(); // Today view by default (full help)
    let width: u16 = 34; // narrow enough to require wrapping but wide enough for any single item
    let lines = help_lines_for_width(&app, width);
    assert!(
        lines.len() > 1,
        "expected multi-line help for narrow width, got: {:?}",
        lines
    );
    for (i, line) in lines.iter().enumerate() {
        let w = UnicodeWidthStr::width(line.as_str()) as u16;
        assert!(w <= width, "line {} exceeds width ({} > {}) => {:?}", i, w, width, line);
    }
}

