#![feature(rustc_attrs)]

struct ReservedDrop;
#[rustc_reservation_impl = "message"]
impl Drop for ReservedDrop {
// { dg-error "" "" { target *-*-* } .-1 }
    fn drop(&mut self) {}
}

fn main() {}

