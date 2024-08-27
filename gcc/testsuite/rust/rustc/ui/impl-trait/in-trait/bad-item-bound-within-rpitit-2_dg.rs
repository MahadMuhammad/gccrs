// issue: 114146

trait Foo {
    fn bar<'other: 'a>() -> impl Sized + 'a {}
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }
}

fn main() {}

