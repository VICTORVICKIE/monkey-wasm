mod utils;

use monkey_lang::*;
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn rows_of_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    return tokens.into_iter().fold(Vec::new(), |mut rows, token| {
        let idx = token.position.line - 1;
        if idx >= rows.len() {
            rows.resize_with(idx + 1, Vec::new);
        }
        rows[idx].push(token);
        rows
    });
}

#[wasm_bindgen]
pub fn lexer(input: String) -> Result<JsValue, JsValue> {
    let lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.into();
    let rows = rows_of_tokens(tokens);

    return Ok(serde_wasm_bindgen::to_value(&rows)?);
}
