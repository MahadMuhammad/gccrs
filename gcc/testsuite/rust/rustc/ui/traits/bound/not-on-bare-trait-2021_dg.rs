// { dg-additional-options "-frust-edition=2021" }
trait Foo {
    fn dummy(&self) {}
}

// This should emit the less confusing error, not the more confusing one.

fn foo(_x: Foo + Send) {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
}
fn bar(x: Foo) -> Foo {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
// { dg-error ".E0782." "" { target *-*-* } .-3 }
    x
}

fn main() {}

