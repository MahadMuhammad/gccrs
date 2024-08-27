//@ compile-flags: -Znext-solver

// Makes sure that alias bounds are not unsound!

#![feature(trivial_bounds)]

trait Foo {
    type Item: Copy
    where
        <Self as Foo>::Item: Copy;

    fn copy_me(x: &Self::Item) -> Self::Item {
        *x
    }
}

impl Foo for () {
    type Item = String where String: Copy;
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}

fn main() {
    let x = String::from("hello, world");
    drop(<() as Foo>::copy_me(&x));
// { dg-error ".E0275." "" { target *-*-* } .-1 }
// { dg-error ".E0275." "" { target *-*-* } .-2 }
// { dg-error ".E0275." "" { target *-*-* } .-3 }
// { dg-error ".E0275." "" { target *-*-* } .-4 }
// { dg-error ".E0275." "" { target *-*-* } .-5 }
// { dg-error ".E0275." "" { target *-*-* } .-6 }
    println!("{x}");
}

