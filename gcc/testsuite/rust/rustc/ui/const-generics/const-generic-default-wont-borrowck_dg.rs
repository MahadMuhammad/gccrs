//@ run-rustfix
pub struct X<const N: usize = {
    let s: &'static str; s.len() // { dg-error ".E0381." "" { target *-*-* } }
}>;

fn main() {}

