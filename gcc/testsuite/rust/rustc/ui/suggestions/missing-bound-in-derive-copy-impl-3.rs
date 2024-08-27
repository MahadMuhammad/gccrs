//@run-rustfix
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Vector2<T: Debug + Copy + Clone>{
    pub x: T,
    pub y: T
}

#[derive(Debug, Copy, Clone)] // { dg-error ".E0204." "" { target *-*-* } }
pub struct AABB<K: Copy>{
    pub loc: Vector2<K>, // { dg-error ".E0277." "" { target *-*-* } }
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    pub size: Vector2<K> // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

