use std::hash::BuildHasher;

fn next_u64() -> u64 {
    let bh = std::hash::RandomState::new();
    let h = bh.build_hasher();
    h.finish() // { dg-error ".E0599." "" { target *-*-* } }
}

trait Bar {}
impl Bar for String {}

fn main() {
    let s = String::from("hey");
    let x: &dyn Bar = &s;
    x.as_ref(); // { dg-error ".E0599." "" { target *-*-* } }
}

