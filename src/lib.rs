mod utils;

use monkey_lang::*;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn lexer(input: String) -> Result<JsValue, JsValue> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.into();

    return Ok(serde_wasm_bindgen::to_value(&tokens)?);
}
