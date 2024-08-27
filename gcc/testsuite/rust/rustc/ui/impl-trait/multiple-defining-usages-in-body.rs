trait Trait {}
impl Trait for () {}

fn foo<T: Trait, U: Trait>() -> impl Trait {
// { dg-warning "" "" { target *-*-* } .-1 }
    let a: T = foo::<T, U>();
    loop {}
    let _: T = foo::<U, T>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

