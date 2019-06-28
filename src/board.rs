use wasm_bindgen::prelude::*;
use crate::domus::*;
use crate::math::random_usize;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellValue {
    O,
    X,
    Empty,
}

impl CellValue {
    pub fn name(&self) -> &str {
        match *self {
            CellValue::O => "O",
            CellValue::X => "X",
            CellValue::Empty => "Empty",
        }
    }
    pub fn is_empty(&self) -> bool {
        match *self {
            CellValue::Empty => true,
            _ => false,
        }
    }
    pub fn next(&self) -> CellValue {
        match *self {
            CellValue::O => CellValue::X,
            CellValue::X => CellValue::O,
            CellValue::Empty => CellValue::Empty,
        }
    }
}

pub struct Board {
    pub cells: [[CellValue;3];3],
    current_player: CellValue,
    move_count: usize,
    pub winner: CellValue,
}

impl Board {
    pub fn play(&mut self, x: usize, y: usize, p: CellValue) -> Result<(), JsValue> {
        if p != self.current_player {
            return js_err("not your turn");
        }
        if !self.cells[x][y].is_empty() {
            return js_err("cell not empty");
        }
        match p {
            CellValue::Empty => js_err("not a player"),
            _ => {
                self.move_count += 1;
                self.cells[x][y] = p;
                if self.is_winning(x, y, p) {
                    log!("We have a winner!");
                    self.winner = p;
                    self.current_player = CellValue::Empty;
                } else {
                    self.current_player = if self.move_count==9 {
                        CellValue::Empty
                    } else {
                        self.current_player.next()
                    };
                }
                Ok(())
            }
        }
    }
    pub fn is_finished(&self) -> bool {
        self.current_player.is_empty()
    }
    pub fn is_winning(&self, x: usize, y: usize, p: CellValue) -> bool {
        let m = Mask::from(self, p);
        match (x, y) {
            (0, 0) => (
                (m.c(0, 1) && m.c(0, 2)) ||
                (m.c(1, 0) && m.c(2, 0)) ||
                (m.c(1, 1) && m.c(2, 2))
            ),
            (0, 1) => (m.c(0, 0) && m.c(0, 2)) || (m.c(1, 1) && m.c(2, 1)),
            (0, 2) => (
                (m.c(0, 0) && m.c(0, 1)) ||
                (m.c(1, 2) && m.c(2, 2)) ||
                (m.c(2, 0) && m.c(1, 1))
            ),
            (1, 0) => (m.c(1, 1) && m.c(1, 2)) || (m.c(0, 0) && m.c(2, 0)),
            (1, 1) => (
                (m.c(0, 0) && m.c(2, 2)) ||
                (m.c(0, 2) && m.c(2, 0)) ||
                (m.c(0, 1) && m.c(2, 1)) ||
                (m.c(1, 0) && m.c(1, 2))
            ),
            (1, 2) => (m.c(1, 0) && m.c(1, 1)) || (m.c(0, 2) && m.c(2, 2)),
            (2, 0) => (
                (m.c(2, 1) && m.c(2, 2)) ||
                (m.c(0, 0) && m.c(1, 0)) ||
                (m.c(1, 1) && m.c(0, 2))
            ),
            (2, 1) => (m.c(2, 0) && m.c(2, 2)) || (m.c(0, 1) && m.c(1, 1)),
            (2, 2) => (
                (m.c(2, 0) && m.c(2, 1)) ||
                (m.c(0, 2) && m.c(1, 2)) ||
                (m.c(0, 0) && m.c(1, 1))
            ),
            (_, _) => { unreachable!() },
        }
    }
    pub fn free_cells(&self) -> Vec<(usize, usize)> {
        let mut free_cells = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if self.cells[x][y].is_empty() {
                    free_cells.push((x, y));
                }
            }
        }
        free_cells
    }
    // assumes there's a possible move
    fn ai_find_move(&mut self) -> (usize, usize) {
        let free_cells = self.free_cells();
        // first let's see if there's a winning move
        for c in &free_cells {
            if self.is_winning(c.0, c.1, self.current_player) {
                return c.clone();
            }
        }
        // now let's see if we can prevent an adverse winning move
        let opponent = self.current_player.next();
        for c in &free_cells {
            if self.is_winning(c.0, c.1, opponent) {
                return c.clone();
            }
        }
        // let's just return any possible move
        free_cells[random_usize(0..free_cells.len())]
    }
    pub fn ai_play(&mut self) {
        if self.is_finished() {
            log!("finished game, AI can't play")
        }
        let ai_move = self.ai_find_move();
        if let Err(_) = self.play(ai_move.0, ai_move.1, self.current_player) {
            log!("AI failed to play!");
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut b = Board {
            cells: [[CellValue::Empty; 3]; 3],
            current_player: CellValue::O,
            move_count: 0,
            winner: CellValue::Empty,
        };
        b.ai_play();
        b
    }
}


// just a stupid utility to type less
struct Mask<'b> {
    cells: &'b [[CellValue;3];3],
    player : CellValue,
}
impl<'b> Mask<'b> {
    fn from(board: &'b Board, player: CellValue) -> Self {
        Mask {
            cells: &board.cells,
            player,
        }
    }
    fn c(&self, x: usize, y: usize) -> bool {
        self.cells[x][y] == self.player
    }
}
