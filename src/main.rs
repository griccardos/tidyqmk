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
[0]=LAYOUT_split_3x6_3(
KC_TAB,KC_Q,KC_W,KC_E,KC_R,KC_T,KC_Y,KC_U,KC_I,KC_O,KC_P,KC_BSPC,
KC_LCTL,KC_A,KC_S,KC_D,KC_F,KC_G,KC_H,KC_J,KC_K,KC_L,KC_SCLN,KC_QUOT,
KC_LSFT,KC_Z,KC_X,KC_C,KC_V,KC_B,KC_N,KC_M,KC_COMM,KC_DOT,KC_SLSH,KC_ESC,
KC_LGUI,TL_LOWR,KC_SPC,KC_ENT,TL_UPPR,KC_RALT

),
      
[1]=LAYOUT_split_3x6_3(
KC_TAB,KC_1,KC_2,KC_3,KC_4,KC_5,KC_6,KC_7,KC_8,KC_9,KC_0,KC_BSPC,
KC_LCTL,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,KC_LEFT,KC_DOWN,KC_UP,KC_RIGHT,XXXXXXX,XXXXXXX,
KC_LSFT,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,XXXXXXX,
KC_LGUI,_______,KC_SPC,KC_ENT,_______,KC_RALT
),
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
    draw_keymap(&keymap, &ops, "/tmp/keymap.svg").unwrap();
}
