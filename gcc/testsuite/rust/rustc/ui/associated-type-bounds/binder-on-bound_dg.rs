trait Trait {
    type Bound<'a>;
}

fn foo() where Trait<for<'a> Bound<'a> = &'a ()> {
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

