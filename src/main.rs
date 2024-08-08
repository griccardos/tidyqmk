use drawsvg::draw_keymap;
use myparser::get_keymap;
use myparser::into_diagnostics;
use myparser::keymap_string;
use myparser::MyParser;
use myparser::Rule;
use options::PrintOptions;
use pest::Parser;

mod drawsvg;
mod error;
mod key;
mod myparser;
mod options;

fn main() {
    let ops = PrintOptions::default();
    let example = r#"
        [0] = LAYOUT_universal(
          KC_Q, KC_W, KC_F, KC_P, KC_B,                                             KC_1, KC_2, KC_U,    KC_Y,   KC_SCLN,
          LSFT_T(KC_A), KC_R, KC_S, KC_T, KC_G,             KC_M, KC_N, KC_E, KC_I,  KC_O,
          KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
          CW_TOGG , QK_REP , KC_DEL, KC_TAB, LT(1,KC_SPC), KC_ESC,   KC_ENT , KC_BSPC,KC_NO,KC_NO,KC_NO,   SCRL_TO
        ),

        [1] = LAYOUT_universal(
          KC_Q, KC_W, KC_E, KC_R, KC_T,                                             KC_1, KC_2, KC_U,    KC_Y,   KC_SCLN,
          KC_A, KC_R, KC_S, KC_T, KC_G,             KC_M, KC_N, KC_E, KC_I,  KC_O,
          KC_Z, KC_X, KC_C, KC_D, KC_V,                                             KC_K, KC_H, KC_COMM, KC_DOT, KC_QUOTE,
          CW_TOGG , QK_REP , KC_DEL, KC_TAB,KC_SPC, KC_ESC,   KC_ENT , KC_BSPC,KC_NO,KC_NO,KC_NO,   SCRL_TO
        )

        "#;

    let prog = match MyParser::parse(Rule::programouter, example) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            println!("{}", into_diagnostics(&e));
            return;
        }
    };
    let keymap = get_keymap(prog, &ops).unwrap();
    let keymap_str = keymap_string(&keymap, &ops);
    println!("{}", keymap_str);
    draw_keymap(&keymap, &ops, "/tmp/my.svg").unwrap();
}
