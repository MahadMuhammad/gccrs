#![feature(type_alias_impl_trait)]

use std::fmt::Display;

type Opaque<X> = impl Sized + 'static;
fn define<X>() -> Opaque<X> {}

trait Trait {
    type Assoc: Display;
}
impl<'a> Trait for Opaque<&'a str> {
// { dg-error ".E0207." "" { target *-*-* } .-1 }
    type Assoc = &'a str;
}

// ======= Exploit =======

fn extend<T: Trait + 'static>(s: T::Assoc) -> Box<dyn Display> {
    Box::new(s)
}

fn main() {
    let val = extend::<Opaque<&'_ str>>(&String::from("blah blah blah"));
    println!("{}", val);
}

