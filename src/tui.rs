use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Direction, Layout}, widgets::{Block, Paragraph, Wrap}
};
use tui_textarea::TextArea;

pub fn tui_run() -> std::io::Result<()> {
    let mut out = String::new();
    let mut terminal = ratatui::init();
    let mut textarea = TextArea::default();
    let block = Block::bordered().title("Base64");
    textarea.set_block(block.clone());
    loop {
        terminal.draw(|frame| {

            let layout = Layout::new(Direction::Horizontal, [
                Constraint::Percentage(50), Constraint::Percentage(50)
            ]).split(frame.area());
            let block = Block::bordered().title("Base64");
            frame.render_widget(block, frame.area());
            frame.render_widget(&textarea, layout[0]);
            frame.render_widget(Paragraph::new(out.as_str()).wrap(Wrap{trim: false}).block(Block::bordered()), layout[1]);
        })?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                textarea.input(key);
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter | KeyCode::Char(_) => {
                        let mut inp = "".to_string();
                        for l in textarea.lines() {
                            inp.push_str(l);
                        }
                        out = rb64::encode(inp.as_bytes());
                    }
                    _ => {},
                }
            }
        }
    }
    ratatui::restore();
    Ok(())
}
