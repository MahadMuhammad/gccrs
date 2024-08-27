// Functions with a type placeholder `_` as the return type should
// show a function pointer suggestion when given a function item
// and suggest how to return closures correctly from a function.
// This is a regression test of #80179

fn returns_i32() -> i32 {
    0
}

fn returns_fn_ptr() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
    returns_i32
}

fn returns_closure() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
// { dg-note ".E0121." "" { target *-*-* } .-5 }
    || 0
}

fn main() {}

