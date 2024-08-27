use std::result;
impl result { // { dg-error ".E0573." "" { target *-*-* } }
    fn into_future() -> Err {} // { dg-error ".E0573." "" { target *-*-* } }
}
fn main() {}

