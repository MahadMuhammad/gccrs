//@ run-rustfix

#![deny(deprecated)]

fn main() {
    let _foo = str::trim_left("   aoeu"); // { dg-error "" "" { target *-*-* } }

    let _bar = "   aoeu".trim_left(); // { dg-error "" "" { target *-*-* } }

    let _baz = ["a", "b"].connect(" "); // { dg-error "" "" { target *-*-* } }
}

