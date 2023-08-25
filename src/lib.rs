mod minesweeper;

use wasm_bindgen::prelude::*;
use minesweeper::*;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String { 
    
    let ms = Minesweeper::new(10, 10, 20);

    ms.to_string()
}

