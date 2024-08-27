enum Foo {
    Bar(Bar),
}
struct Bar {
    x: i32,
}

fn a(f: Foo) {
    match f {
        Bar { x } => {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
        }
    }
}

struct S;

fn b(s: Option<S>) {
    match s {
        S => {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
        }
        _ => {}
    }
}

fn c(s: Result<S, S>) {
    match s {
        S => {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
        }
        _ => {}
    }
}

fn main() {}

