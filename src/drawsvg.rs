use svg::{
    node::element::{Group, Rectangle, Text},
    Document,
};

use crate::{key::nice_code, Keymap, PrintOptions};

pub fn draw_keymap(
    keymap: &Keymap,
    ops: &PrintOptions,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_width = 100;
    let key_height = 50;
    let padding = 50;
    let centre_width = 50;
    let full_width = keymap.layers.first().unwrap().keys.first().unwrap().len() * key_width
        + padding * 2
        + centre_width;
    let layer_height = keymap.layers.first().unwrap().keys.len() * key_height + padding * 2;
    let full_height = layer_height * keymap.layers.len();
    let mut groups = vec![];

    for (layi, layer) in keymap.layers.iter().enumerate() {
        let grid = &layer.keys;
        let mut group = Group::new();
        let layer_name = if layer.num.len() == 1 {
            format!("Layer {}", layer.num)
        } else {
            layer.num.clone()
        };
        let header = Text::new(layer_name)
            .set("x", 30)
            .set("y", layi * layer_height + padding / 2)
            .set("text-anchor", "left")
            .set("font-size", "30")
            .set("dominant-baseline", "middle");
        group = group.add(header);
        let surround = Rectangle::new()
            .set("x", 10)
            .set("y", layi * layer_height)
            .set("width", full_width - 20)
            .set("height", layer_height)
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1);
        group = group.add(surround);
        let centre = grid.first().unwrap().len() / 2;
        for (li, line) in grid.iter().enumerate() {
            for (i, code) in line.iter().enumerate() {
                let mut x = i * key_width + padding;
                if i >= centre {
                    x += centre_width;
                }
                let y = layi * layer_height + li * key_height + padding;
                if let Some(code) = code {
                    let rect = Rectangle::new()
                        .set("x", x)
                        .set("y", y)
                        .set("width", key_width)
                        .set("height", key_height)
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 1);
                    group = group.add(rect);
                    let nice = nice_code(code);
                    let text_middle = Text::new(nice.middle)
                        .set("x", x + key_width / 2)
                        .set("y", y + key_height / 2)
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "middle");
                    group = group.add(text_middle);
                    let text_top = Text::new(nice.top)
                        .set("x", x + key_width / 2)
                        .set("y", y + 10)
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "middle");
                    group = group.add(text_top);
                    let text_bottom = Text::new(nice.bottom)
                        .set("x", x + key_width / 2)
                        .set("y", y + key_height - 10)
                        .set("text-anchor", "middle")
                        .set("dominant-baseline", "middle");
                    group = group.add(text_bottom);
                }
            }
        }
        groups.push(group);
    }
    let mut doc = Document::new().set("viewBox", (0, 0, full_width, full_height));
    for group in groups {
        doc = doc.add(group);
    }

    svg::save(path, &doc)?;
    Ok(())
}
