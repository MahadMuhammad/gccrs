fn foo(x: Option<i32>) {
    if true && x {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

fn main() {}

