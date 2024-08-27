#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

pub struct Vec3;

#[const_trait]
pub trait Add {
    fn add(self) -> Vec3;
}

impl Add for Vec3 {
    const fn add(self) -> Vec3 {
// { dg-error ".E0379." "" { target *-*-* } .-1 }
        self
    }
}

fn main() {}

