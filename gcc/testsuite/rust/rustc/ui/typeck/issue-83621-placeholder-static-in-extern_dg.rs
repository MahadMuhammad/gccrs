// Regression test for #83621.

extern "C" {
    static x: _; // { dg-error ".E0121." "" { target *-*-* } }
}

fn main() {}

