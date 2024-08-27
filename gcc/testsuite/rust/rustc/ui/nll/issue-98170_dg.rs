pub struct MyStruct<'a> {
    field: &'a [u32],
}

impl MyStruct<'_> {
    pub fn new<'a>(field: &'a [u32]) -> MyStruct<'a> {
        Self { field }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

trait Trait<'a> {
    fn new(field: &'a [u32]) -> MyStruct<'a>;
}

impl<'a> Trait<'a> for MyStruct<'_> {
    fn new(field: &'a [u32]) -> MyStruct<'a> {
        Self { field }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

fn main() {}

