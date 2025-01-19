use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Paragraph, Wrap},
};
use tui_textarea::TextArea;

pub fn tui_run() -> std::io::Result<()> {
    let mut out = String::new();
    let mut terminal = ratatui::init();
    let mut textarea = TextArea::default();
    let block = Block::bordered().title("Base64");
    textarea.set_block(block.clone());
    'main: loop {
        terminal.draw(|frame| {
            let layout = Layout::new(Direction::Horizontal, [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());
            let block = Block::bordered().title("Base64");
            frame.render_widget(block, frame.area());
            frame.render_widget(&textarea, layout[0]);
            frame.render_widget(
                Paragraph::new(out.as_str())
                    .wrap(Wrap { trim: false })
                    .block(Block::bordered()),
                layout[1],
            );
        })?;
        let mut must_update = false;
        while event::poll(Duration::from_secs(0)).unwrap_or(false) {
            let Event::Key(key) = event::read()? else { continue };

            textarea.input(key);
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Esc => break 'main,
                    KeyCode::Delete | KeyCode::Enter
                    | KeyCode::Backspace | KeyCode::Char(_) => {
                            must_update = true;
                        }
                    _ => {}
                }
            }
        }
        if must_update {
            let mut inp = "".to_string();
            /* TODO: Replace with Iterator::intersperse when stabilized */
            let mut first = true;
            for l in textarea.lines() {
                if !first {
                    inp.push('\n');
                }
                first = false;
                inp.push_str(l);
            }
            out = rb64::encode(inp.as_bytes());
        }
    }
    ratatui::restore();
    Ok(())
}
