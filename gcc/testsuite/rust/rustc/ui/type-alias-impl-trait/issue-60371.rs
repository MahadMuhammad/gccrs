trait Bug {
    type Item: Bug;

    const FUN: fn() -> Self::Item;
}

impl Bug for &() {
    type Item = impl Bug; // { dg-error ".E0658." "" { target *-*-* } }

    const FUN: fn() -> Self::Item = || ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

