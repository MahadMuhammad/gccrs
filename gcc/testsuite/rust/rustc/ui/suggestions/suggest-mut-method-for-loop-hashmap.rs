//@ run-rustfix
// https://github.com/rust-lang/rust/issues/82081

use std::collections::HashMap;

struct Test {
    v: u32,
}

fn main() {
    let mut map = HashMap::new();
    map.insert("a", Test { v: 0 });

    for (_k, v) in map.iter() {
// { help "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        v.v += 1;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
// { dg-note ".E0594." "" { target *-*-* } .-2 }
    }
}

