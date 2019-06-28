use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};
use std::sync::{Arc, Mutex};
use crate::board::{Board, CellValue};
use crate::domus::*;

/// manages the DOM and event handlers for a board
pub struct BoardView {
    id: String, // the id of the board DOM element
    board: Board,
    player: CellValue,
}

struct EventHandlerSet {
    cell_handlers: Vec<Closure<dyn FnMut()>>,
    new_game_handler: Option<Closure<dyn FnMut()>>,
}

impl Drop for EventHandlerSet {
    fn drop(&mut self){
        // I hope I'll see message this when I'll replace the board_view by another one
        log!("event handlers dropped");
    }
}

// This helps keep getting the callbacks alive without
//  (completely) leaking them.
// It's a "singleton", meaning there can be only one
//  working board view at a time.
static mut LAST_HANDLERS: Option<EventHandlerSet> = None;

impl BoardView {
    pub fn new(id: String) -> Self {
        BoardView {
            id,
            board: Board::default(),
            player: CellValue::X,
        }
    }
    pub fn reset_board(&mut self) {
        self.board = Board::default();
        self.redraw_cells();
    }
    pub fn cell_id(&self, x: usize, y: usize) -> String {
        format!("cell_{}_{}_{}", &self.id, x, y)
    }
    pub fn redraw_cells(&self) {
        for i in 0..3 {
            for j in 0..3 {
                if let Some(cell) = doc().get_element_by_id(&self.cell_id(i, j)) {
                    cell.set_class_name(&format!("cell {}", self.board.cells[i][j].name()));
                } else {
                    log!("I lost cell {} {}!", i, j);
                }
            }
        }
    }
    fn show_new_game_button(&mut self, view: &Arc<Mutex<BoardView>>) {
        let panel = tag_class("div", "board-panel").unwrap();
        let button = tag_class("div", "button").unwrap();
        button.set_inner_text("New Game");
        panel.append_child(&button).unwrap();
        if let Some(b) = doc().get_element_by_id(&self.id) {
            b.append_child(&panel).unwrap();
        } else {
            log!("I lost the board");
            return;
        }
        let closure_view = Arc::clone(view);
        let on_click = Closure::wrap(Box::new(move|| {
            log!("new game");
            let mut view = closure_view.lock().unwrap();
            view.reset_board();
            remove_by_selector(&format!("#{} .game-outcome", view.id));
            remove_by_selector(&format!("#{} .board-panel", view.id));
            // let's release the event handler from memory
            unsafe {
                if let Some(ref mut handlers) = LAST_HANDLERS {
                    handlers.new_game_handler = None;
                }
            }
        }) as Box<dyn FnMut()>);
        button.set_onclick(Some(on_click.as_ref().unchecked_ref()));
        unsafe {
            if let Some(ref mut handlers) = LAST_HANDLERS {
                handlers.new_game_handler = Some(on_click);
            }
        }
    }
    fn show_game_outcome(&self, s: &str) {
        if let Some(footer) = doc().get_element_by_id("footer") {
            footer.set_class_name("visible");
        }
        let m: HtmlElement = tag_class("div", "game-outcome").unwrap();
        m.set_inner_text(s);
        if let Some(b) = doc().get_element_by_id(&self.id) {
            b.append_child(&m).unwrap();
        } else {
            log!("I lost the board");
        }
    }
    pub fn run_in(self, container: &HtmlElement) -> Result<(), JsValue> {
        let be = tag_class("div", "board")?;
        be.set_attribute("id", &self.id)?;
        container.append_child(&be)?;
        let view = Arc::new(Mutex::new(self));
        let mut handlers = EventHandlerSet {
            cell_handlers: Vec::new(),
            new_game_handler: None,
        };
        for i in 0..3 {
            for j in 0..3 {
                let cell: HtmlElement = tag_class("div", "cell")?;
                {
                    let view = view.lock().unwrap(); // must be done everytime ?
                    let cell_id = view.cell_id(i, j);
                    cell.set_attribute("id", &cell_id)?;
                    cell.class_list().add_1(view.board.cells[i][j].name())?;
                };
                be.append_child(&cell)?;
                let closure_view = Arc::clone(&view);
                let on_click = Closure::wrap(Box::new(move|| {
                    log!("clicked {} {}", i, j);
                    let mut view = closure_view.lock().unwrap();
                    let player = view.player;
                    if let Err(e) = view.board.play(i, j, player) {
                        log!("Illegal move: {:?}", e);
                        return;
                    }
                    view.redraw_cells();
                    if !view.board.winner.is_empty() {
                        view.show_game_outcome("You win!");
                    } else {
                        view.board.ai_play();
                        view.redraw_cells();
                        if !view.board.winner.is_empty() {
                            view.show_game_outcome("You lose!");
                        }
                    }
                    if view.board.is_finished() {
                        if view.board.winner.is_empty() {
                            view.show_game_outcome("Both lose");
                        }
                        view.show_new_game_button(&closure_view);
                    }
                }) as Box<dyn FnMut()>);
                cell.set_onclick(Some(on_click.as_ref().unchecked_ref()));
                handlers.cell_handlers.push(on_click);
            }
        }
        unsafe {
            LAST_HANDLERS = Some(handlers);
        }
        Ok(())
    }
}


