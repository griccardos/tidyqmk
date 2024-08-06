use pest::Parser;
use pest_derive::Parser;

fn main() {
    let example = r#"
     KC_Q, KC_W, KC_F, KC_P, KC_B,                                             KC_J, KC_L, KC_U,    KC_Y,   KC_SCLN, 
    LSFT_T(KC_A), LCTL_T(KC_R), LALT_T(KC_S), LGUI_T(KC_T), KC_G,             KC_M, RGUI_T(KC_N), RALT_T(KC_E), RCTL_T(KC_I), RSFT_T( KC_O),
    KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
    CW_TOGG , QK_REP , KC_DEL, LT(2,KC_TAB) , LT(1,KC_SPACE), LT(3,KC_ESC),   LSFT_T(KC_ENT) , LT(2,KC_BSPC),KC_NO,KC_NO,KC_NO,   SCRL_TO
    "#;
    let example = r#"
      KC_Q, KC_W, KC_F, KC_P, KC_B,                                             KC_J, KC_L, KC_U,    KC_Y,   KC_SCLN, 
    LSFT_T(KC_A), LCTL_T(KC_R), LALT_T(KC_S), LGUI_T(KC_T), KC_G,             KC_M, RGUI_T(KC_N), RALT_T(KC_E), RCTL_T(KC_I), RSFT_T( KC_O),
    KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
   CW_TOGG , QK_REP 
   "#;

    let pairs = MyParser::parse(Rule::program, example).unwrap();

    for pair in pairs {
        println!("{:?}", pair);
    }
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

        let pairs = MyParser::parse(Rule::keycode, example).unwrap();

        assert_eq!(pairs.count(), 1);
    }

    #[test]
    fn test_grammar_keycode_transform() {
        let example = "LSFT_T(KC_A)";

        let pairs = MyParser::parse(Rule::keycode, example).unwrap();

        assert_eq!(pairs.count(), 1);
    }

    #[test]
    fn test_grammar_layer_transform() {
        let example = "LT(2,KC_TAB)";

        let pairs = MyParser::parse(Rule::keycode, example).unwrap();

        assert_eq!(pairs.count(), 1);
    }

    #[test]
    fn multiple_keycodes() {
        let example = r#"KC_Q, KC_W, KC_F, KC_P, KC_B,"#;
        let pairs = MyParser::parse(Rule::line, example).unwrap();
        assert_eq!(pairs.count(), 1);
    }

    #[test]
    fn keycode_multiple() {
        let example = r#"KC_Q, KC_W, KC_F, KC_P, KC_B,"#;
        let mut pairs = MyParser::parse(Rule::line, example).unwrap();
        let pairs = pairs.nth(0).unwrap().into_inner();
        let mut keycodes = Vec::new();
        for pair in pairs {
            keycodes.push(pair.as_str());
        }
        assert_eq!(keycodes, vec!["KC_Q", "KC_W", "KC_F", "KC_P", "KC_B"]);
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
    fn keycode_function() {
        let example = r#" LT(1,KC_NO) "#;
        let pairs = MyParser::parse(Rule::program, example).unwrap();
    }

    /*  #[test]
    fn test_grammar_program() {
        let example = r#"
            KC_Q, KC_W,
             KC_Q, KC_W,
            KC_Z, LT(2,KC_TAB),
            CW_TOGG ,KC_W
        "#;

        let pairs = MyParser::parse(Rule::program, example).unwrap();

        assert_eq!(pairs.count(), 1);
    }*/
}
