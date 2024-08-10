use easier::prelude::*;
use pest::Parser;

use crate::{myparser::format_pair, MyParser, PrintOptions, Rule};

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

    fn mt(mid: &str, top: &str) -> PrintKey {
        Self {
            top: top.to_string(),
            middle: mid.to_string(),
            bottom: String::new(),
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
            _ => {
                println!("Error Trying to parse a {:?}", parsed.as_rule());
                KeyType::KeyCode(format!("{:?}", parsed.as_rule()))
            }
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
            bottom: format!("L-{}", params[0]),
        }
    } else if (name == "LSFT_T" || name == "RSFT_T" || name == "SFT_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "⇧".to_string(),
        }
    } else if (name == "LCTL_T" || name == "RCTL_T" || name == "CTL_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "⌃".to_string(),
        }
    } else if (name == "LALT_T" || name == "RALT_T" || name == "ALT_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "⌥".to_string(),
        }
    } else if (name == "LGUI_T" || name == "RGUI_T" || name == "GUI_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "⌘".to_string(),
        }
    } else if (name == "MEH_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "MEH".to_string(),
        }
    } else if (name == "HYPR" || name == "ALL_T") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey {
            top: tap.top,
            middle: tap.middle,
            bottom: "HYPR".to_string(),
        }
    } else if (name == "S" || name == "LSFT") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey::mt(&format!("⇧{}", tap.middle), &tap.top)
    } else if (name == "C" || name == "LCTL") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey::mt(&format!("⌃{}", tap.middle), &tap.top)
    } else if (name == "A" || name == "LALT") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey::mt(&format!("⌥{}", tap.middle), &tap.top)
    } else if (name == "G" || name == "LGUI") && params.len() == 1 {
        let tap = nice_code(&params[0]);
        PrintKey::mt(&format!("⌘{}", tap.middle), &tap.top)
    } else {
        PrintKey::new(raw)
    }
}

fn nice_keycode(code: &str) -> PrintKey {
    match code {
        "KC_UP" => PrintKey::new("↑"),
        "KC_DOWN" => PrintKey::new("↓"),
        "KC_LEFT" => PrintKey::new("←"),
        "KC_RGHT" | "KC_RIGHT" => PrintKey::new("→"),
        "KC_NO" | "XXXXXXX" => PrintKey::new(""),
        "KC_DOT" => PrintKey::mt(".", ">"),
        "KC_COMM" => PrintKey::mt(",", "<"),
        "KC_SLSH" => PrintKey::mt("/", "?"),
        "KC_BSLS" => PrintKey::mt("\\", "|"),
        "KC_ENT" => PrintKey::new("↵"),
        "KC_BSPC" => PrintKey::new("⌫"),
        "KC_SPC" => PrintKey::new("␣"),
        "KC_TAB" => PrintKey::new("⇥"),
        "KC_DEL" => PrintKey::new("⌦"),
        "KC_GRV" => PrintKey::mt("`", "~"),
        "KC_LBRC" => PrintKey::mt("[", "{"),
        "KC_RBRC" => PrintKey::mt("]", "}"),
        "KC_SCLN" => PrintKey::mt(";", ":"),
        "KC_EQL" => PrintKey::mt("=", "+"),
        "KC_MINS" => PrintKey::mt("-", "_"),
        "KC_CAPS" => PrintKey::new("⇪"),
        "KC_QUOT" | "KC_QUOTE" => PrintKey::mt("'", "\""),
        "KC_LCTL" | "KC_RCTL" => PrintKey::new("⌃"),
        "KC_LSFT" | "KC_RSFT" => PrintKey::new("⇧"),
        "KC_LALT" | "KC_RALT" => PrintKey::new("⌥"),
        "KC_LGUI" | "KC_RGUI" => PrintKey::new("⌘"),
        "KC_1" => PrintKey::mt("1", "!"),
        "KC_2" => PrintKey::mt("2", "@"),
        "KC_3" => PrintKey::mt("3", "#"),
        "KC_4" => PrintKey::mt("4", "$"),
        "KC_5" => PrintKey::mt("5", "%"),
        "KC_6" => PrintKey::mt("6", "^"),
        "KC_7" => PrintKey::mt("7", "&"),
        "KC_8" => PrintKey::mt("8", "*"),
        "KC_9" => PrintKey::mt("9", "("),
        "KC_0" => PrintKey::mt("0", ")"),
        "KC_TRNS" | "_______" => PrintKey::new("⇄"),
        "KC_PDOT" => PrintKey::mt(".", "⌦"),
        "KC_P1" => PrintKey::mt("1", "End"),
        "KC_P2" => PrintKey::mt("2", "↓"),
        "KC_P3" => PrintKey::mt("3", "PgDn"),
        "KC_P4" => PrintKey::mt("4", "←"),
        "KC_P5" => PrintKey::mt("5", "Clear"),
        "KC_P6" => PrintKey::mt("6", "→"),
        "KC_P7" => PrintKey::mt("7", "Home"),
        "KC_P8" => PrintKey::mt("8", "↑"),
        "KC_P9" => PrintKey::mt("9", "PgUp"),
        "KC_P0" => PrintKey::mt("0", "Ins"),
        "KC_PPLS" => PrintKey::new("+"),
        "KC_PMNS" => PrintKey::new("-"),
        "KC_PAST" => PrintKey::new("*"),
        "KC_PSLS" => PrintKey::new("/"),
        "KC_PEQL" => PrintKey::new("="),

        //shifted versions
        "KC_EXLM" => PrintKey::new("!"),
        "KC_AT" => PrintKey::new("@"),
        "KC_HASH" => PrintKey::new("#"),
        "KC_DLR" => PrintKey::new("$"),
        "KC_PERC" => PrintKey::new("%"),
        "KC_CIRC" => PrintKey::new("^"),
        "KC_AMPR" => PrintKey::new("&"),
        "KC_ASTR" => PrintKey::new("*"),
        "KC_LPRN" => PrintKey::new("("),
        "KC_RPRN" => PrintKey::new(")"),
        "KC_TILD" => PrintKey::new("~"),
        "KC_LCBR" => PrintKey::new("{"),
        "KC_RCBR" => PrintKey::new("}"),
        "KC_COLN" => PrintKey::new(":"),
        "KC_PLUS" => PrintKey::new("+"),
        "KC_UNDS" => PrintKey::new("_"),
        "KC_PIPE" => PrintKey::new("|"),
        "KC_LT" => PrintKey::new("<"),
        "KC_GT" => PrintKey::new(">"),
        "KC_QUES" => PrintKey::new("?"),
        "KC_DQUO" => PrintKey::new("\""),

        _ if code.starts_with("KC_") => {
            let part2 = code.split_once('_').unwrap().1;
            PrintKey::new(part2)
        }
        _ => PrintKey::new(code),
    }
}
