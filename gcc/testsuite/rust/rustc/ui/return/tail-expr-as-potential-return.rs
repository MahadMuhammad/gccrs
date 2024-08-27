// { dg-additional-options "-frust-edition=2018" }

// > Suggest returning tail expressions that match return type
// >
// > Some newcomers are confused by the behavior of tail expressions,
// > interpreting that "leaving out the `;` makes it the return value".
// > To help them go in the right direction, suggest using return instead
// > when applicable.
// (original commit description for this test)
//
// This test was amended to also serve as a regression test for #92308, where
// this suggestion would not trigger with async functions.

fn main() {
}

fn foo(x: bool) -> Result<f64, i32> {
    if x {
        Err(42)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    }
    Ok(42.0)
}

async fn bar(x: bool) -> Result<f64, i32> {
    if x {
        Err(42)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    }
    Ok(42.0)
}

trait Identity {
    type Out;
}

impl<T> Identity for T {
    type Out = T;
}

async fn foo2() -> i32 {
    if true {
        1i32
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    }
    0
}

struct Receiver;
impl Receiver {
    fn generic<T>(self) -> Option<T> {
        None
    }
}
fn method() -> Option<i32> {
    if true {
        Receiver.generic();
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { help ".E0282." "" { target *-*-* } .-2 }
// { help ".E0282." "" { target *-*-* } .-3 }
    }

    None
}

