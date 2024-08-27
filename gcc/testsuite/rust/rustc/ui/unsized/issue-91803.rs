trait Foo<'a> {}

fn or<'a>(first: &'static dyn Foo<'a>) -> dyn Foo<'a> {
// { dg-error ".E0746." "" { target *-*-* } .-1 }
    return Box::new(panic!());
}

fn main() {}

