use std::collections::HashMap;
struct X(usize);
struct Y {
    v: u32,
}

fn main() {
    let _ = || {
        let mut buzz = HashMap::new();
        buzz.insert("a", Y { v: 0 });

        for mut t in buzz.values() {
// { help "" "" { target *-*-* } .-1 }
// { suggestion "" "" { target *-*-* } .-2 }
            t.v += 1;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
        }
    };
}

