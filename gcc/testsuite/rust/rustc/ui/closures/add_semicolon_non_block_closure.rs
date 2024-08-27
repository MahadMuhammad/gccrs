fn foo(_f: impl Fn()) {}

fn bar() -> i32 {
    1
}

fn main() {
    foo(|| bar())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
}

