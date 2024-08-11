# Tidyqmk

Help to manage your keymap.c layout by changing this:

``` c
 [0]=LAYOUT_split_3x6_3(
 KC_Q,KC_W,KC_E,KC_R,KC_T,KC_Y,KC_U,KC_I,KC_O,KC_P,
 KC_A,KC_S,KC_D,KC_F,KC_G,KC_H,KC_J,KC_K,KC_L,KC_SCLN,
 KC_Z,KC_X,KC_C,KC_V,KC_B,KC_N,KC_M,KC_COMM,KC_DOT,KC_SLSH,
 KC_LGUI,TL_LOWR,KC_SPC,KC_ENT,TL_UPPR,KC_RALT
 
 ),
```

to 

``` c
[0] = LAYOUT_split_3x6_3 (
 KC_Q, KC_W, KC_E,    KC_R,    KC_T,                     KC_Y    ,KC_U    ,KC_I    ,KC_O   ,KC_P    ,
 KC_A, KC_S, KC_D,    KC_F,    KC_G,                     KC_H    ,KC_J    ,KC_K    ,KC_L   ,KC_SCLN ,
 KC_Z, KC_X, KC_C,    KC_V,    KC_B,                     KC_N    ,KC_M    ,KC_COMM ,KC_DOT ,KC_SLSH ,
                   KC_LGUI, TL_LOWR, KC_SPC,     KC_ENT ,TL_UPPR ,KC_RALT                           
),

```

- Supports multiple layers
- Supports multiple thumb rows
- Aligns multiple layers together (or separately if you wish)
- Works with split and non split
- Shift thumb keys


## To use
- copy your badly formatted keymap.c layout
- copy the cleaned code back
