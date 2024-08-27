//@ compile-flags: -C debug-assertions

fn main() {
    env!{"\t"}; // { dg-error "" "" { target *-*-* } }
    env!("\t"); // { dg-error "" "" { target *-*-* } }
    env!("\u{2069}"); // { dg-error "" "" { target *-*-* } }
}

