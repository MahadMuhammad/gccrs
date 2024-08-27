#![feature(type_alias_impl_trait)]
trait OpaqueTrait {}
impl<T> OpaqueTrait for T {}
type OpaqueType = impl OpaqueTrait;
fn mk_opaque() -> OpaqueType {
    || 0
}
trait AnotherTrait {}
impl<T: Send> AnotherTrait for T {}
impl AnotherTrait for OpaqueType {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }
fn main() {}

