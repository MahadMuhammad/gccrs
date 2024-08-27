// Test that encountering closures during coherence does not cause issues.
#![feature(type_alias_impl_trait)]
type OpaqueClosure = impl Sized;
fn defining_use() -> OpaqueClosure {
    || ()
}

struct Wrapper<T>(T);
trait Trait {}
impl Trait for Wrapper<OpaqueClosure> {}
impl<T: Sync> Trait for Wrapper<T> {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

fn main() {}

