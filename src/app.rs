use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement};

use crate::board::Board;
use crate::board_view::BoardView;
use crate::domus::*;

pub struct App {
    board_view: BoardView,
}

impl App {
    pub fn new() -> Self {
        let board = Board::default();
        let board_view = BoardView::new("main".to_owned(), board);
        Self {
            board_view,
        }
    }
    // initial construction of the page
    pub fn run_inside(self, parent: &mut HtmlElement) -> Result<(), JsValue> {
        log!("Hi! This isn't a JavaScript application but a Wasm application written in Rust. Play a game to know more.");
        //parent.empty();
        let bbox = tag_class("div", "board-box")?;
        self.board_view.run_in(&bbox)?;
        parent.append_child(&bbox)?;
        Ok(())
    }
}
