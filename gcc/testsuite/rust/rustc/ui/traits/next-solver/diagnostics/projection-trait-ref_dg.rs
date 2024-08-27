//@ compile-flags: -Znext-solver

trait Trait {
    type Assoc;
}

fn test_poly<T>() {
    let x: <T as Trait>::Assoc = ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn test() {
    let x: <i32 as Trait>::Assoc = ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

