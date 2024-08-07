use easier::prelude::*;
use pest::Parser;

use crate::{format_pair, MyParser, PrintOptions, Rule};

#[derive(Default)]
pub struct PrintKey {
    pub top: String,
    pub middle: String,
    pub bottom: String,
}
impl PrintKey {
    pub fn new(middle: &str) -> Self {
        Self {
            top: String::new(),
            middle: middle.to_string(),
            bottom: String::new(),
        }
    }
    pub fn mb(middle: &str, bottom: &str) -> Self {
        Self {
            top: String::new(),
            middle: middle.to_string(),
            bottom: bottom.to_string(),
        }
    }
}

enum KeyType {
    KeyCode(String),
    Function(String, Vec<String>, String),
}
impl From<&str> for KeyType {
    fn from(value: &str) -> Self {
        let parsed = MyParser::parse(Rule::key, value)
            .unwrap()
            .next() //from multiple to 1
            .unwrap()
            .into_inner() //go to keycode or function
            .next() //there is only one
            .unwrap();
        match parsed.as_rule() {
            Rule::keycode => KeyType::KeyCode(parsed.as_str().to_string()),
            Rule::function => {
                let raw = format_pair(parsed.clone(), &PrintOptions::default());
                let mut inner = parsed.into_inner();
                let out = inner.next().unwrap().as_str().to_string();
                let params = inner.next().unwrap().into_inner();

                KeyType::Function(
                    out,
                    params.into_iter().map(|a| a.as_str().to_string()).to_vec(),
                    raw,
                )
            }
            _ => unreachable!("Trying to parse a {:?}", parsed.as_rule()),
        }
    }
}

pub fn nice_code(code: &str) -> PrintKey {
    let t: KeyType = code.into();

    match t {
        KeyType::KeyCode(code) => nice_keycode(&code),
        KeyType::Function(name, params, raw) => nice_function(&name, &params, &raw),
    }
}

fn nice_function(name: &str, params: &Vec<String>, raw: &str) -> PrintKey {
    if name == "LT" && params.len() == 2 {
        let tap = nice_code(&params[1]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: format!("L{}", params[0]),
        }
    } else {
        PrintKey::new(raw)
    }
}

fn nice_keycode(code: &str) -> PrintKey {
    match code {
        "KC_UP" => PrintKey::new("↑"),
        "KC_DOWN" => PrintKey::new("↓"),
        "KC_LEFT" => PrintKey::new("←"),
        "KC_RGHT" => PrintKey::new("→"),
        "KC_NO" => PrintKey::new(""),
        "KC_DOT" => PrintKey::new("."),
        "KC_COMM" => PrintKey::new(","),
        "KC_SLSH" => PrintKey::new("/"),
        "KC_BSLS" => PrintKey::new("\\"),
        "KC_QUOTE" => PrintKey {
            top: "\"".to_string(),
            middle: "'".to_string(),
            bottom: "".to_string(),
        },
        "KC_ENT" => PrintKey::new("↵"),
        "KC_BSPC" => PrintKey::new("⌫"),
        "KC_SPC" => PrintKey::new("␣"),
        "KC_TAB" => PrintKey::new("⇥"),
        "KC_DEL" => PrintKey::new("⌦"),
        "KC_1" => PrintKey {
            top: "!".to_string(),
            middle: "1".to_string(),
            bottom: "".to_string(),
        },

        _ if code.starts_with("KC_") => {
            let part2 = code.split_once('_').unwrap().1;
            PrintKey::new(part2)
        }
        _ => PrintKey::new(code),
    }
}
