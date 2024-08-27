// Regression test for #88844.

struct Struct { value: i32 }
// { dg-note "" "" { target *-*-* } .-1 }

impl Stuct {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { help ".E0412." "" { target *-*-* } .-2 }
    fn new() -> Self {
        Self { value: 42 }
    }
}

fn main() {}

