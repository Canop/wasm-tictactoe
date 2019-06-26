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

/// A game move,
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
    pub x: usize,
    pub y: usize,
    pub player: CellValue,
}

pub struct Board {
    pub cells: [[CellValue;3];3],
    current_player: CellValue,
    move_count: usize,
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
                self.current_player = if self.move_count==9 {
                    CellValue::Empty
                } else {
                    self.current_player.next()
                };
                Ok(())
            }
        }
    }
    pub fn is_finished(&self) -> bool {
        self.current_player.is_empty()
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
    pub fn ai_play(&mut self) {
        if self.is_finished() {
            log!("finished game, AI can't play")
        }
        let free_cells = self.free_cells();
        let i: usize = random_usize(free_cells.len());
        if let Err(e) = self.play(free_cells[i].0, free_cells[i].1, self.current_player) {
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
        };
        b.ai_play();
        b
    }
}
