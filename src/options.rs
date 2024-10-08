use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PrintOptions {
    pub thumb_shift_in: usize,
    pub left_align: bool,
    pub split_space: usize,
    pub align_layers: bool,
    pub number_of_thumbs: usize,
    pub draw_nice: bool,
}
#[wasm_bindgen]
impl PrintOptions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PrintOptions {
        Default::default()
    }
}

impl Default for PrintOptions {
    fn default() -> Self {
        PrintOptions {
            thumb_shift_in: 1,
            left_align: false,
            split_space: 5,
            align_layers: true,
            number_of_thumbs: 1,
            draw_nice: false,
        }
    }
}
