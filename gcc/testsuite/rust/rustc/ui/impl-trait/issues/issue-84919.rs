trait Trait {}
impl Trait for () {}

fn foo<'a: 'a>() {
    let _x: impl Trait = ();
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

fn main() {}

