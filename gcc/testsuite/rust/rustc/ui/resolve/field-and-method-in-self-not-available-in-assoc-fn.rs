struct Foo {
    field: u32,
}

impl Foo {
    fn field(&self) -> u32 {
        self.field
    }

    fn new() -> Foo {
        field; // { dg-error ".E0425." "" { target *-*-* } }
        Foo { field } // { dg-error ".E0425." "" { target *-*-* } }
    }
    fn clone(&self) -> Foo {
        Foo { field } // { dg-error ".E0425." "" { target *-*-* } }
    }
}
fn main() {}

