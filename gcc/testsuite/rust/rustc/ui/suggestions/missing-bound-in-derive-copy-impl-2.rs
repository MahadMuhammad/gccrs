//@ run-rustfix
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: Debug + Copy + Clone> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone)]
pub struct AABB<K: Debug> {
    pub loc: Vector2<K>, // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    pub size: Vector2<K>, // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

