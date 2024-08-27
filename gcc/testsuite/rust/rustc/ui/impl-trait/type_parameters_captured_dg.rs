use std::fmt::Debug;

trait Any {}
impl<T> Any for T {}

// Check that type parameters are captured and not considered 'static
fn foo<T>(x: T) -> impl Any + 'static {
    x
// { dg-error ".E0310." "" { target *-*-* } .-1 }
}

fn main() {}

