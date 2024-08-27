// Checks what happens when `public` is used instead of the correct, `pub`
//@ run-rustfix

public enum Test {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    A,
    B,
}

fn main() { }

