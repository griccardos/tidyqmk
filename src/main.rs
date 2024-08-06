use pest::Parser;
use pest_derive::Parser;

fn main() {
    let example = r#"
     KC_Q, KC_W, KC_F, KC_P, KC_B,                                             KC_J, KC_L, KC_U,    KC_Y,   KC_SCLN,
    LSFT_T(KC_A), LCTL_T(KC_R), LALT_T(KC_S), LGUI_T(KC_T), KC_G,             KC_M, RGUI_T(KC_N), RALT_T(KC_E), RCTL_T(KC_I), RSFT_T( KC_O),
    KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
    CW_TOGG , QK_REP , KC_DEL, LT(2,KC_TAB) , LT(1,KC_SPACE), LT(3,KC_ESC),   LSFT_T(KC_ENT) , LT(2,KC_BSPC),KC_NO,KC_NO,KC_NO,   SCRL_TO
    "#;

    let lines = MyParser::parse(Rule::program, example).unwrap();

    let mut line_codes = vec![];
    for line in lines {
        let mut keycodes = vec![];
        for keycode in line.into_inner() {
            keycodes.push(format(keycode))
        }
        line_codes.push(keycodes);
    }
    let max_cols = line_codes.iter().map(|x| x.len()).max().unwrap();
    let max_len = line_codes.iter().fold(
        std::iter::repeat(0).take(max_cols).collect(),
        |acc: Vec<usize>, line| {
            acc.iter()
                .zip(line.iter().chain(std::iter::repeat(&"".to_string()))) //need to pad shorter cols
                .map(|(a, l)| l.len().max(*a))
                .collect()
        },
    );
    for line in line_codes {
        for (i, code) in line.iter().enumerate() {
            print!("{: <1$},", code, max_len[i] + 1);
        }
        println!();
    }
}

fn format(pair: pest::iterators::Pair<Rule>) -> String {
    let mut result = String::new();
    //println!("key is {:?}", pair);
    match pair.as_rule() {
        Rule::key => {
            result.push_str(&format(pair.into_inner().next().unwrap()));
        }
        Rule::keycode => {
            result.push_str(pair.as_str());
        }
        Rule::function => {
            let mut inner_pair = pair.into_inner();
            let function_name = inner_pair.next().unwrap();
            result.push_str(&format(function_name));
            result.push_str("(");
            let params = inner_pair.next().unwrap();
            result.push_str(&format(params));
            result.push_str(")");
        }
        Rule::param => {
            result.push_str(pair.as_str());
        }
        Rule::params => {
            let mut params = Vec::new();
            let inner = pair.into_inner();
            for code in inner {
                params.push(format(code));
            }
            result.push_str(params.join(",").as_str());
        }
        Rule::program => {}
        Rule::WHITESPACE => {}
        Rule::EOI => {}
        Rule::line => {} //not used, because we operate on the codes in it
    }
    result
}

#[derive(Parser)]
#[grammar = "qmk.pest"]
struct MyParser;
#[cfg(test)]
mod tests {
    use pest::iterators::Pairs;

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

        let pairs = MyParser::parse(Rule::program, example).unwrap();
        println!("{:?}", pairs);
        assert_eq!(pairs.count(), 3); //includes EOI
    }

    #[test]
    fn test_grammar_program() {
        let example = r#"
            KC_Q, KC_W,
             KC_Q, KC_W,
            KC_Z, LT(2,KC_TAB),
            CW_TOGG ,KC_W
        "#;

        let pairs = MyParser::parse(Rule::program, example).unwrap();
        println!("{pairs:?}");
        assert_eq!(pairs.count(), 4 + 1); //includes EOI
    }

    #[test]
    fn print_keycodes() {
        let example = r#"KC_Q"#;

        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        assert_eq!("KC_Q", format(pairs.next().unwrap()))
    }

    #[test]
    fn print_function() {
        let example = r#"LT(1,KC_NO)"#;

        let mut pairs = MyParser::parse(Rule::key, example).unwrap();
        assert_eq!("LT(1,KC_NO)", format(pairs.next().unwrap()))
    }
}
