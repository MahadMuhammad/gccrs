//@ run-rustfix

trait Trait {}
impl Trait for () {}

// this works
fn foo() -> impl Trait {
    ()
}

fn bar<T: Trait + std::marker::Sync>() -> T
where
    T: Send,
{
    () // { dg-error ".E0308." "" { target *-*-* } }
}

fn other_bounds<T>() -> T
where
    T: Trait,
    Vec<usize>: Clone,
{
    () // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {
    foo();
    bar::<()>();
    other_bounds::<()>();
}

