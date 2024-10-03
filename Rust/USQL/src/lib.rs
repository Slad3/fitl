mod data_structures;
mod tokenize;
mod compile;
mod filter;
mod table;

use wasm_bindgen::prelude::*;

use crate::tokenize::tokenize;
use crate::compile::compile;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}




#[wasm_bindgen(start)]
pub fn run() {
    console_log!("Start!");
}