// Type ascription doesn't lead to unsoundness

#![feature(type_ascription)]

fn main() {
    let arr = &[1u8, 2, 3];
    let ref x = type_ascribe!(arr, &[u8]);      // { dg-error ".E0308." "" { target *-*-* } }
    let ref mut x = type_ascribe!(arr, &[u8]);  // { dg-error ".E0308." "" { target *-*-* } }
    match type_ascribe!(arr, &[u8]) {           // { dg-error ".E0308." "" { target *-*-* } }
        ref x => {}
    }
    let _len = type_ascribe!(arr, &[u8]).len();              // { dg-error ".E0308." "" { target *-*-* } }
}

