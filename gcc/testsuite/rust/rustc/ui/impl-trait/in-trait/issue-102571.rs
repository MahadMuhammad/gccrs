use std::fmt::Display;
use std::ops::Deref;

trait Foo {
    fn bar(self) -> impl Deref<Target = impl Display + ?Sized>;
}

fn foo<T: Foo>(t: T) {
    let () = t.bar();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

