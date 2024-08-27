// { dg-additional-options "-frust-edition= 2021" }

async fn a() -> i32 {
    0
}

fn foo() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
    a()
}

fn bar() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
    async { a().await }
}

fn main() {}

