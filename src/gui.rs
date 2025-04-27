use std::cell::RefCell;
use std::fmt::Write;
use std::rc::Rc;

use fltk::app::Scheme;
use fltk::dialog::FileChooserType;
use fltk::enums::{Align, CallbackTrigger, Event, Shortcut};
use fltk::frame::Frame;
use fltk::menu::{MenuBar, MenuFlag};
use fltk::{prelude::*, *};
use rb64;

#[derive(Clone,Copy)]
enum Op {
    Encode,
    Decode,
}

struct State {
    op: Op,
    status: Frame,
}

impl State {
    fn switch_mode(&mut self, mode: Op) {
        self.op = mode;
        self.set_label(None);
    }

    fn get_label_for_mode(&self) -> &'static str {
        match self.op {
            Op::Encode => "[Encode]",
            Op::Decode => "[Decode]",
        }
    }

    fn set_label<'a>(&mut self, extra: impl Into<Option<&'a str>>) {
        let mut status = self.get_label_for_mode().to_string();
        if let Some(e) = extra.into() {
            write!(status, ": {e}").unwrap();
        }
        self.status.set_label(&status);
    }
}

#[derive(Clone)]
struct Splitter {
    left: text::TextEditor,
    right: text::TextDisplay,
    divider: frame::Frame,
    dragging: bool,
    state: Rc<RefCell<State>>,
}

impl Splitter {
    fn new(x: i32, y: i32, w: i32, h: i32, state: Rc<RefCell<State>>) -> Self {
        let mut left = text::TextEditor::new(x, y, w / 2 - 2, h, None);
        left.set_buffer(text::TextBuffer::default());
        left.set_trigger(CallbackTrigger::Changed);

        let divider = frame::Frame::new(x + w / 2 - 2, y, 4, h, None);
        /* divider.set_frame(FrameType::FlatBox); */

        let mut right = text::TextDisplay::new(x + w / 2 + 2, y, w / 2 - 2, h, None);
        right.set_buffer(text::TextBuffer::default());
        right.wrap_mode(text::WrapMode::AtBounds, 0);

        let mut spl = Self {
            left,
            right,
            divider,
            state,
            dragging: false,
        };

        spl.left.set_callback({
            let spl = spl.clone();
            move |_| {
                spl.process()
            }
        });

        spl
    }

    fn handle_event(&mut self, ev: Event) -> bool {
        match ev {
            Event::Push => {
                self.dragging = true;
                true
            }
            Event::Drag if self.dragging => {
                let x = app::event_coords().0;
                let min_x = self.left.x() + 50;
                let max_x = self.right.x() + self.right.w() - 50;

                if x >= min_x && x <= max_x {
                    let left_w = x - self.left.x();
                    let right_w = self.right.x() + self.right.w() - x - 4;

                    self.left.resize(self.left.x(), self.left.y(), left_w, self.left.h());
                    self.divider.resize(x, self.divider.y(), 4, self.divider.h());
                    self.right.resize(x + 4, self.right.y(), right_w, self.right.h());

                    self.left.redraw();
                    self.divider.redraw();
                    self.right.redraw();
                }
                true
            }
            Event::Released => {
                self.dragging = false;
                true
            }
            _ => false,
        }
    }

    fn process(&self) {
        if let Some(buf) = self.left.buffer() {
            let text = buf.text();
            let op = self.state.borrow().op;
            self.state.borrow_mut().set_label(None);
            let text = match op {
                Op::Encode => rb64::encode(text.as_bytes()),
                Op::Decode => {
                    match rb64::decode(&text) {
                        Ok(dec) => String::from_utf8(dec.into()).unwrap_or_else(|err| {
                            let err = format!("ERROR: {err}");
                            self.state.borrow_mut().set_label(Some(err.as_str()));
                            "".to_string()
                        }),
                        Err(err) => {
                            let err = format!("ERROR: {err}");
                            self.state.borrow_mut().set_label(Some(err.as_ref()));
                            "".to_string()
                        }
                    }
                }
            };
            self.right.buffer().unwrap().set_text(&text);
        }
    }

    fn switch(&self) {
        let r = self.right.buffer().unwrap().text();
        self.left.buffer().unwrap().set_text(&r);
        self.right.buffer().unwrap().set_text("");
        self.process();
    }
}

const MENU_HEIGHT: i32 = 25;
const INITIAL_WIDTH: i32 = 800;
const INITIAL_HEIGHT: i32 = 600;
const STATUS_HEIGHT: i32 = 25;

fn choose_file(ty: FileChooserType, title: &str) -> Option<String> {
    let mut chooser = dialog::FileChooser::new(
        ".",
        "*",
        ty,
        title,
    );
    chooser.show();
    chooser.window().set_pos(300, 300);

    while chooser.shown() {
        app::wait();
    }

    chooser.value(1)
}

fn menu_bar(state: Rc<RefCell<State>>, splitter: Splitter) {
    let mut menu = MenuBar::new(0, 0, INITIAL_WIDTH, MENU_HEIGHT, "");

    menu.add(
        "File/Open",
        Shortcut::Ctrl | Shortcut::from_char('o'),
        MenuFlag::Normal,
        {
            let s = splitter.clone();
            move |_| {
                if let Some(f) = choose_file(FileChooserType::Single, "Open file") {
                    let Ok(text) = std::fs::read_to_string(&f) else {
                        s.state.borrow_mut().set_label("ERROR: Non utf8 files are not supported right now");
                        return
                    };
                    s.left.buffer().unwrap().set_text(&text);
                    s.process();
                }
            }
        }
    );

    menu.add(
        "File/Save",
        Shortcut::Ctrl | Shortcut::from_char('s'),
        MenuFlag::Normal,
        {
            let s = splitter.clone();
            move |_| {
                if let Some(f) = choose_file(FileChooserType::Create, "Save as") {
                    let text = s.right.buffer().unwrap().text();
                    let msg = match std::fs::write(&f, &text) {
                        Ok(_) => format!("Saved to {f}"),
                        Err(err) => format!("Error saving file: {err}"),
                    };
                    s.state.borrow_mut().set_label(msg.as_str());
                }
            }
        }
    );

    menu.add(
        "File/Exit",
        Shortcut::None,
        MenuFlag::Normal,
        move |_| {
            std::process::exit(0);
        }
    );

    menu.add(
        "Mode/Encode",
        Shortcut::None,
        MenuFlag::Normal,
        {
            let s = Rc::clone(&state);
            let spl = splitter.clone();
            move |_| {
                s.borrow_mut().switch_mode(Op::Encode);
                spl.switch();
            }
        }
    );

    menu.add(
        "Mode/Decode",
        Shortcut::None,
        MenuFlag::Normal,
        {
            move |_| {
                state.borrow_mut().switch_mode(Op::Decode);
                splitter.switch();
            }
        }
    );
}

pub fn start_gui() -> Result<(), String> {
    let app = app::App::default().with_scheme(Scheme::Base);

    let mut win = window::Window::default()
        .with_size(INITIAL_WIDTH, INITIAL_HEIGHT)
        .with_label("Base64")
        .center_screen();

    let mut status = Frame::default()
                       .with_pos(10, INITIAL_HEIGHT - STATUS_HEIGHT)
                       .with_size(790, STATUS_HEIGHT)
                       .with_label("[Encode]");

    status.set_align(Align::Left | Align::Inside);

    let state = State {
        op: Op::Encode,
        status,
    };
    let state = Rc::new(RefCell::new(state));

    let splitter = Splitter::new(
        10,
        MENU_HEIGHT,
        INITIAL_WIDTH - 10,
        INITIAL_HEIGHT - MENU_HEIGHT - STATUS_HEIGHT,
        Rc::clone(&state),
    );

    menu_bar(Rc::clone(&state), splitter.clone());

    win.handle({
        let mut splitter = splitter.clone();
        move |_, ev| splitter.handle_event(ev)
    });

    win.resizable(&splitter.left);
    win.end();
    win.show();

    app.run().map_err(|err| {
        format!("Couldn't start GUI: {err}")
    })
}
