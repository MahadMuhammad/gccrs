// Test that the parser detects `&dyn mut`, offers a help message, and
// recovers.

fn main() {
    let r: &dyn mut Trait;
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { help ".E0405." "" { target *-*-* } .-2 }
// { dg-error ".E0405." "" { target *-*-* } .-3 }
}

