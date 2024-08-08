use myparser::{get_keymap, into_diagnostics, MyParser, PrintOptions, Rule};
use pest::Parser;
use wasm_bindgen::prelude::*;

mod drawsvg;
mod key;
mod myparser;
#[wasm_bindgen]
pub fn generate_svg(example: &str) -> Vec<String> {
    let ops = PrintOptions::default();

    let prog = match MyParser::parse(Rule::programouter, example) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            println!("{}", into_diagnostics(e));
            return vec!["".to_string(), "".to_string()];
        }
    };
    let keymap = get_keymap(prog, &ops);
    let svg = crate::drawsvg::generate_svg(&keymap, &ops);
    let out = crate::myparser::generate_keymap(&keymap, &ops);
    vec![svg, out]
}
/*
#[wasm_bindgen]
pub fn generate_svg() -> String {
    let document = Document::new().set("viewBox", (0, 0, 100, 100)).add(
        Circle::new()
            .set("cx", 50)
            .set("cy", 50)
            .set("r", 40)
            .set("fill", "red"),
    );

    let mut buffer = Vec::new();
    svg::write(&mut buffer, &document).unwrap();
    String::from_utf8(buffer).unwrap()
}
*/
