trait T {
    extern "Rust" unsafe fn foo();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
}

fn main() {}

