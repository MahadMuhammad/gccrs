#![deny(dead_code)]

struct T1; // { dg-error "" "" { target *-*-* } }
pub struct T2(i32); // { dg-error "" "" { target *-*-* } }
struct T3;

trait Trait1 { // { dg-error "" "" { target *-*-* } }
    const UNUSED: i32;
    fn unused(&self) {}
    fn construct_self() -> Self;
}

pub trait Trait2 {
    const USED: i32;
    fn used(&self) {}
}

pub trait Trait3 {
    const USED: i32;
    fn construct_self() -> Self;
}

impl Trait1 for T1 {
    const UNUSED: i32 = 0;
    fn construct_self() -> Self {
        Self
    }
}

impl Trait1 for T2 {
    const UNUSED: i32 = 0;
    fn construct_self() -> Self {
        T2(0)
    }
}

impl Trait2 for T1 {
    const USED: i32 = 0;
}

impl Trait2 for T2 {
    const USED: i32 = 0;
}

impl Trait3 for T3 {
    const USED: i32 = 0;
    fn construct_self() -> Self {
        Self
    }
}

fn main() {}

