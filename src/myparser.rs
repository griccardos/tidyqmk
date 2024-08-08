use easier::prelude::*;
use pest::{
    error::{Error, ErrorVariant},
    iterators::Pair,
};
use pest_derive::Parser;

use crate::{error::MyError, options::PrintOptions};

pub struct Keymap {
    pub layers: Vec<Layer>,
}
pub struct Layer {
    pub num: String,
    name: String,
    pub keys: Vec<Vec<Option<String>>>,
}

pub fn get_keymap(pair: Pair<Rule>, ops: &PrintOptions) -> Result<Keymap, MyError> {
    assert!(pair.as_rule() == Rule::program);
    let mut keymap = Keymap { layers: Vec::new() };
    //we go through layers
    let inner = pair.into_inner();
    for block in inner {
        keymap.layers.push(get_layer(block, ops)?);
    }

    //check all layers have the same number of keys
    let num_keys = keymap.layers[0].keys.len();
    for layer in &keymap.layers {
        if layer.keys.len() != num_keys {
            return Err("All layers must have the same number of keys"
                .to_string()
                .into());
        }
    }

    Ok(keymap)
}
fn get_layer(pair: Pair<Rule>, ops: &PrintOptions) -> Result<Layer, MyError> {
    let mut inner = pair.into_inner();
    let num = inner.next().unwrap().as_str().to_string();
    let name = inner.next().unwrap().as_str().to_string();
    let lines = inner.next().unwrap().into_inner();

    let mut line_codes = vec![];
    for line in lines {
        let mut keycodes = vec![];
        for keycode in line.into_inner() {
            keycodes.push(format_pair(keycode, ops))
        }
        line_codes.push(keycodes);
    }
    validate(&line_codes)?;
    let grid = create_grid(line_codes, ops);
    Ok(Layer {
        keys: grid,
        num,
        name,
    })
}

pub fn format_pair(pair: pest::iterators::Pair<Rule>, ops: &PrintOptions) -> String {
    let mut result = String::new();
    //println!("key is {:?}", pair);
    match pair.as_rule() {
        Rule::key => {
            result.push_str(&format_pair(pair.into_inner().next().unwrap(), ops));
        }
        Rule::keycode => {
            result.push_str(pair.as_str());
        }
        Rule::function => {
            let mut inner_pair = pair.into_inner();
            let function_name = inner_pair.next().unwrap();
            result.push_str(&format_pair(function_name, ops));
            result.push_str("(");
            let params = inner_pair.next().unwrap();
            result.push_str(&format_pair(params, ops));
            result.push_str(")");
        }
        Rule::param => {
            result.push_str(pair.as_str());
        }
        Rule::params => {
            let mut params = Vec::new();
            let inner = pair.into_inner();
            for code in inner {
                params.push(format_pair(code, ops));
            }
            result.push_str(params.join(",").as_str());
        }
        Rule::layerblock => {
            let mut inner = pair.into_inner();
            let layernum = inner.next().unwrap();
            let layer = format_pair(layernum, ops);
            let layercmd = inner.next().unwrap().as_str();
            result.push_str(&format!(
                "[{layer}] = {layercmd}(\n{}\n)",
                format_pair(inner.next().unwrap(), ops)
            ));
        }
        Rule::program => {} //dont use

        Rule::layer => {} //dont use
        Rule::layernum => {
            result.push_str(pair.as_str());
        }
        Rule::validname => {
            result.push_str(pair.as_str());
        }
        //not used, because we operate on the codes in it
        Rule::line => {}
        //these are all ignored
        Rule::WHITESPACE => {}
        Rule::COMMENT => {}
        Rule::multicomment => {}
        Rule::linecomment => {}
        Rule::EOI => {}
        Rule::programouter => {}
        Rule::number => {}
        Rule::white => {}
    }
    result
}

fn validate(line_codes: &[Vec<String>]) -> Result<(), MyError> {
    //at least one row
    if line_codes.is_empty() {
        return Err("No rows".to_string().into());
    }
    // //even number of cols for each row
    // if !line_codes.iter().all(|a| a.len() % 2 == 0) {
    //     return Err("Odd number of columns in a row".to_string().into());
    // }
    Ok(())
}

//for split, we want to start in centre and work our way out
//if we shift in thumb cols, we want to add space to each other row on the inside
//we align each column to the centre and add padding to outside if needed
//if uneven number of keys, we may have to pad keys
fn create_grid(line_codes: Vec<Vec<String>>, ops: &PrintOptions) -> Vec<Vec<Option<String>>> {
    let rows = line_codes.len();
    let mut grid = line_codes
        .into_iter()
        .map(|a| a.into_iter().map(|b| Some(b)).to_vec())
        .to_vec();

    //make sure even number in grid (sometimes less columns on one side)
    for line in grid.iter_mut() {
        if line.len() % 2 != 0 {
            line.push(None);
        }
    }

    //if thumb shift, we add to each row inside except thumb row
    for (li, line) in grid.iter_mut().enumerate() {
        if li < rows - ops.number_of_thumbs {
            for _ in 0..ops.thumb_shift_in * 2 {
                //add to centre
                let centre = line.len() / 2;
                line.insert(centre, None);
            }
        }
    }

    let max_cols = grid.iter().map(|x| x.len()).max().unwrap();
    for line in grid.iter_mut() {
        while line.len() < max_cols {
            line.push(None);
            line.insert(0, None);
        }
    }

    grid
}

pub fn keymap_string(keymap: &Keymap, ops: &PrintOptions) -> String {
    let column_count = keymap.layers[0].keys[0].len();

    println!("{:?} cols:{}", keymap.layers[0].keys, column_count);

    let column_layer_lens = keymap
        .layers
        .iter()
        .map(|layer| {
            layer.keys.iter().fold(
                std::iter::repeat(0).take(column_count).collect(),
                |acc: Vec<usize>, line| {
                    acc.iter()
                        .zip(line.iter())
                        .map(|(a, l)| l.as_ref().map(|a| a.len()).unwrap_or_default().max(*a))
                        .collect()
                },
            )
        })
        .collect::<Vec<_>>();
    let mut result = String::new();

    for (layi, layer) in keymap.layers.iter().enumerate() {
        let grid = &layer.keys;
        result.push_str(&format!("[{}] = {} (\n", layer.num, layer.name));

        for (li, line) in grid.iter().enumerate() {
            println!("line is {:?}", line);
            println!("column_layer_lens is {:?}", column_layer_lens);

            for (i, code) in line.iter().enumerate() {
                let max_len = if ops.align_layers {
                    column_layer_lens.iter().map(|x| x[i]).max().unwrap()
                } else {
                    column_layer_lens[layi][i]
                };
                let width = max_len + 1;
                let centre = line.len() / 2;
                let mut comma = ",";
                //check if there are no more buttons after this
                if li == grid.len() - 1 {
                    if line.iter().skip(i + 1).all(|x| x.is_none()) {
                        comma = "";
                    }
                };
                match code {
                    Some(code) => {
                        if i >= centre {
                            result.push_str(&format!("{: <1$}{comma}", code, width));
                        } else {
                            if ops.left_align {
                                result.push_str(&format!("{: <1$}{comma}", code, width));
                            } else {
                                result.push_str(&format!("{: >1$}{comma}", code, width));
                            }
                        }
                    }
                    None => result.push_str(&format!("{: ^1$}", "  ", width + 1)), //+1 for the comma that is missing here
                }

                if i == centre - 1 {
                    let space = std::iter::repeat(" ")
                        .take(ops.split_space)
                        .collect::<String>();
                    result.push_str(&space);
                }
            }
            result.push_str("\n");
        }
        result.push_str(")");
        result.push_str(",\n");
    }
    result
}

pub fn into_diagnostics(e: &Error<Rule>) -> String {
    match &e.variant {
        ErrorVariant::ParsingError {
            positives,
            negatives,
        } => {
            let mut message = format!("Parsing error at {:?}", e.line_col);
            if !positives.is_empty() {
                message.push_str(" (expected ");
                message.push_str(
                    positives
                        .iter()
                        .map(|s| format!("{:#?}", s))
                        .collect::<Vec<String>>()
                        .join(" or ")
                        .as_str(),
                );
                message.push(')');
            }

            if !negatives.is_empty() {
                message.push_str(" (unexpected ");
                message.push_str(
                    negatives
                        .iter()
                        .map(|s| format!("\"{:#?}\"", s))
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str(),
                );
                message.push(')');
            }
            message
        }
        _ => "Unknown error".to_owned(),
    }
}

#[derive(Parser)]
#[grammar = "qmk.pest"]
pub struct MyParser;

#[cfg(test)]
mod tests {

    use pest::Parser;

    use super::*;

    #[test]
    fn test_grammar_keycode() {
        let example = "KC_A";
        let mut pairs = MyParser::parse(Rule::keycode, example).unwrap();
        assert_eq!(pairs.clone().count(), 1);
        assert_eq!(pairs.next().unwrap().as_str(), "KC_A")
    }

    #[test]
    fn function1() {
        let example = "LSFT_T(KC_A)";
        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.clone().count(), 1);
        assert_eq!(pairs.next().unwrap().as_str(), "LSFT_T(KC_A)")
    }

    #[test]
    fn function2() {
        let example = "LT(2,KC_TAB)";
        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.clone().count(), 1);
        assert_eq!(pairs.next().unwrap().as_str(), "LT(2,KC_TAB)")
    }

    #[test]
    fn multiple_keycodes() {
        let example = r#"KC_Q, KC_W, KC_F, KC_P, KC_B,"#;
        let mut pairs = MyParser::parse(Rule::line, example).unwrap();
        assert_eq!(pairs.clone().count(), 1);
        let pairs = pairs.nth(0).unwrap().into_inner();
        let mut keycodes = Vec::new();
        for pair in pairs {
            keycodes.push(pair.as_str());
        }
        assert_eq!(keycodes, vec!["KC_Q", "KC_W", "KC_F", "KC_P", "KC_B"]);
    }

    #[test]
    fn multiple_keycodes_mixed() {
        let example = r#"KC_Q,  S(KC_F),LT(1,KC_W),"#;
        let mut pairs = MyParser::parse(Rule::line, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.nth(0).unwrap().into_inner().count(), 3);
    }

    #[test]
    fn multiple_keycodes_nocomma() {
        let example = r#"KC_Q, KC_W, KC_F, KC_P, KC_B"#;
        let mut pairs = MyParser::parse(Rule::line, example).unwrap();
        assert_eq!(pairs.clone().count(), 1);
        assert_eq!(pairs.nth(0).unwrap().into_inner().count(), 5);
    }

    #[test]
    fn two_lines() {
        let example = r#"KC_Q, KC_W,
       KC_Q, KC_W,
        "#;

        let pairs = MyParser::parse(Rule::layer, example)
            .unwrap()
            .nth(0)
            .unwrap()
            .into_inner();
        println!("{:?}", pairs);
        assert_eq!(pairs.count(), 2);
    }

    #[test]
    fn test_grammar_layer() {
        let example = r#"
            KC_Q, KC_W,
             KC_Q, KC_W,
            KC_Z, LT(2,KC_TAB),
            CW_TOGG ,KC_W
        "#;

        let pairs = MyParser::parse(Rule::layer, example)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();
        println!("{pairs:?}");
        assert_eq!(pairs.count(), 4);
    }

    #[test]
    fn print_keycodes() {
        let example = r#"KC_Q"#;

        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        assert_eq!(
            "KC_Q",
            format_pair(pairs.next().unwrap(), &PrintOptions::default())
        )
    }

    #[test]
    fn print_function() {
        let example = r#"LT(1,KC_NO)"#;

        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        assert_eq!(
            "LT(1,KC_NO)",
            format_pair(pairs.next().unwrap(), &PrintOptions::default())
        )
    }

    #[test]
    fn test_layer() {
        let example = r#"[0] = LAYOUT_universal(
              KC_Q, KC_W, KC_F, KC_P, KC_B,                                             KC_1, KC_2, KC_U,    KC_Y,   KC_SCLN,
              LSFT_T(KC_A), KC_R, KC_S, KC_T, KC_G,             KC_M, KC_N, KC_E, KC_I,  KC_O,
              KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
              CW_TOGG , QK_REP , KC_DEL, KC_TAB, LT(1,KC_SPC), KC_ESC,   KC_ENT , KC_BSPC,KC_NO,KC_NO,KC_NO,   SCRL_TO
              )"#;

        let mut pairs = MyParser::parse(Rule::layerblock, example)
            .unwrap()
            .next()
            .unwrap()
            .into_inner();

        let number = pairs.next().unwrap().as_str();
        assert_eq!(number, "0");
        let name = pairs.next().unwrap().as_str();
        assert_eq!(name, "LAYOUT_universal");
        let lines = pairs.next().unwrap().into_inner();
        assert_eq!(lines.clone().count(), 4);
    }

    #[test]
    fn test_program() {
        let example = r#"
            [0] = LAYOUT_universal( KC_A,KC_B,
            KC_Q, KC_W,
            ),
            [1] = LAYOUT_universal( KC_B)

        "#;
        let pairs = MyParser::parse(Rule::programouter, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.count(), 2);
    }

    #[test]
    fn test_comments() {
        let example = r#"
            // This is a comment
            [0] = LAYOUT_universal( KC_A,KC_B,
            KC_Q,/*something
            other
            */ KC_W,
            ),
            [1] = LAYOUT_universal( KC_B)

        "#;
        let pairs = MyParser::parse(Rule::programouter, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.count(), 2);
    }

    #[test]
    fn test_uneven() {
        let example = r#"
            [0] = LAYOUT_universal( KC_A,KC_B,
            KC_Q
            ),
            [1] = LAYOUT_universal( KC_B,KC_B
            KC_1
            )

        "#;
        let ops = PrintOptions::default();
        let mut prog = MyParser::parse(Rule::programouter, example).unwrap();
        let keymap = get_keymap(prog.next().unwrap(), &ops).unwrap();
        keymap_string(&keymap, &ops);
    }
}
