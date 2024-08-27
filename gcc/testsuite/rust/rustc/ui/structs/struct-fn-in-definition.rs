// It might be intuitive for a user coming from languages like Java
// to declare a method directly in a struct's definition. Make sure
// rustc can give a helpful suggestion.
// Suggested in issue #76421

struct S {
    field: usize,

    fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

union U {
    variant: usize,

    fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

enum E {
    Variant,

    fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
}

fn main() {}

