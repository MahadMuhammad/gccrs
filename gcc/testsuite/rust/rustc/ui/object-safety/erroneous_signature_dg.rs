trait Foo {
    fn err(&self) -> MissingType;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl Foo for i32 {
    fn err(&self) -> MissingType {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
        0
    }
}

fn coerce(x: &i32) -> &dyn Foo {
    x
}

fn main() {}

