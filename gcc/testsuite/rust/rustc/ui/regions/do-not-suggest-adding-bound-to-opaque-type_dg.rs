pub trait T {}

struct S<'a>(&'a ());

impl<'a> T for S<'a> {}

fn foo() -> impl T {
    let x = ();
    S(&x) // { dg-error ".E0597." "" { target *-*-* } }
}

fn main() {}

