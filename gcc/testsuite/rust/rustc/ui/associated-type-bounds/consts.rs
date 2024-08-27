pub fn accept(_: impl Trait<K: Copy>) {}
// { dg-error "" "" { target *-*-* } .-1 }

pub trait Trait {
    const K: i32;
}

fn main() {}

