use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::board::{Board, CellValue};
use crate::domus::*;

pub struct BoardView {
    id: String, // the id of the board DOM element
    board: Board,
    player: CellValue,
}

impl BoardView {
    pub fn new(id: String, board: Board) -> Self {
        BoardView {
            id,
            board,
            player: CellValue::X,
        }
    }
    pub fn cell_id(&self, x: usize, y: usize) -> String {
        format!("cell_{}_{}_{}", &self.id, x, y)
    }
    pub fn redraw_cells(&self) {
        for i in 0..3 {
            for j in 0..3 {
                //let cell_id = view.cell_id(i, j);
                if let Some(cell) = doc().get_element_by_id(&self.cell_id(i, j)) {
                    cell.set_class_name(&format!("cell {}", self.board.cells[i][j].name()));
                } else {
                    log!("I lost cell {} {}!", i, j);
                }
            }
        }
    }
    pub fn run_in(self, container: &HtmlElement) -> Result<(), JsValue> {
        let be = tag_class("div", "board")?;
        be.set_attribute("id", &self.id)?;
        container.append_child(&be)?;
        let view = Arc::new(Mutex::new(self));
        for i in 0..3 {
            for j in 0..3 {
                let cell: HtmlElement = tag_class("div", "cell")?;
                {
                    let mut view = view.lock().unwrap(); // must be done everytime ?
                    let cell_id = view.cell_id(i, j);
                    cell.set_attribute("id", &cell_id)?;
                    cell.class_list().add_1(view.board.cells[i][j].name())?;
                };
                be.append_child(&cell)?;
                let mut closure_view = Arc::clone(&view);
                let on_click = Closure::wrap(Box::new(move|| {
                    log!("clicked {} {}", i, j);
                    let mut view = closure_view.lock().unwrap();
                    let player = view.player;
                    if let Err(e) = view.board.play(i, j, player) {
                        log!("Illegal move: {:?}", e);
                        return;
                    }
                    view.board.ai_play();
                    view.redraw_cells();
                    //if let Some(cell) = doc().get_element_by_id(&cell_id) {
                    //    cell.set_class_name(view.board.cells[i][j].name());
                    //} else {
                    //    log!("I lost my cell!"); // should not really happen as it was just clicked
                    //}
                }) as Box<dyn FnMut()>);
                cell.set_onclick(Some(on_click.as_ref().unchecked_ref()));
                // by using `forget` we prevent it from dying on end of scope
                // but we leak the closure.
                // Doing otherwise would be
                // - storing the closure in the board (or a board_view)
                // - ensuring the board_view stays alive (which looks like
                //   the hard part)
                on_click.forget();
            }
        }
        Ok(())
    }
}


