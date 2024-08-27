use std::ops::Deref;

struct Foo;

impl Deref for Foo {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &[]
    }
}

fn main() {
    let _ = "foo" as bool;
// { dg-error ".E0606." "" { target *-*-* } .-1 }

    let _ = String::from("foo") as bool;
// { dg-error ".E0605." "" { target *-*-* } .-1 }

    let _ = Foo as bool;
// { dg-error ".E0605." "" { target *-*-* } .-1 }
}

fn _slice(bar: &[i32]) -> bool {
    bar as bool
// { dg-error ".E0606." "" { target *-*-* } .-1 }
}

