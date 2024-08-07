use svg::{node::element::Circle, Document};
use wasm_bindgen::prelude::*;
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
