use myparser::{get_keymap, into_diagnostics, MyParser, Rule};
use options::PrintOptions;
use pest::Parser;
use wasm_bindgen::prelude::*;

mod drawsvg;
mod error;
mod key;
mod myparser;
mod options;
#[wasm_bindgen]
pub fn generate_svg(example: &str, ops: PrintOptions) -> Vec<String> {
    let prog = match MyParser::parse(Rule::programouter, example) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            println!("{}", into_diagnostics(&e));
            return vec!["".to_string(), "".to_string(), e.to_string()];
        }
    };
    let keymap = match get_keymap(prog, &ops) {
        Ok(k) => k,
        Err(e) => {
            return vec!["".to_string(), "".to_string(), e.to_string()];
        }
    };
    let svg = crate::drawsvg::create_svg(&keymap, &ops);
    let out = crate::myparser::keymap_string(&keymap, &ops);
    vec![svg, out, "".to_string()]
}
