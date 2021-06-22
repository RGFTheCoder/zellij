use crate::tab::Pane;
use super::super::TerminalPane;
use ::insta::assert_snapshot;
use zellij_utils::{zellij_tile::data::Palette};
use zellij_utils::pane_size::PositionAndSize;

#[test]
pub fn scrolling_inside_a_pane() {
    let fake_win_size = PositionAndSize {
        cols: 121,
        rows: 20,
        x: 0,
        y: 0,
        ..Default::default()
    };
    let pid = 1;
    let palette = Palette::default();
    let mut terminal_pane = TerminalPane::new(pid, fake_win_size, palette);
    let mut text_to_fill_pane = String::new();
    for i in 0..30 {
        text_to_fill_pane.push_str(&format!("\rline {}\n", i + 1));
    }
    terminal_pane.handle_pty_bytes(text_to_fill_pane.as_bytes().to_vec());
    terminal_pane.scroll_up(10);
    assert_snapshot!(format!("{:?}", terminal_pane.grid));
    terminal_pane.scroll_down(3);
    assert_snapshot!(format!("{:?}", terminal_pane.grid));
    terminal_pane.clear_scroll();
    assert_snapshot!(format!("{:?}", terminal_pane.grid));
}
