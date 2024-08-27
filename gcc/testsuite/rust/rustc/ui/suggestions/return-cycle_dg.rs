use std::marker::PhantomData;

struct Token<T>(PhantomData<T>);

impl<T> Token<T> {
    fn new() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
        Token(PhantomData::<()>)
    }
}

fn main() {}

