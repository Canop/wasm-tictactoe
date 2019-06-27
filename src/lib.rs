
#[macro_use]
mod domus;

mod app;
mod board;
mod board_view;
mod math;

use wasm_bindgen::prelude::*;

/// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let app = app::App::new();
    let mut body = domus::body();
    app.run_inside(&mut body)?;
    Ok(())
}

