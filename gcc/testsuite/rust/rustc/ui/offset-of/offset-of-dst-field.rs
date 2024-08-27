#![feature(extern_types)]

use std::mem::offset_of;

struct Alpha {
    x: u8,
    y: u16,
    z: [u8],
}

trait Trait {}

struct Beta {
    x: u8,
    y: u16,
    z: dyn Trait,
}

extern {
    type Extern;
}

struct Gamma {
    x: u8,
    y: u16,
    z: Extern,
}

struct Delta<T: ?Sized> {
    x: u8,
    y: u16,
    z: T,
}

fn main() {
    offset_of!(Alpha, z); // { dg-error ".E0277." "" { target *-*-* } }
    offset_of!(Beta, z); // { dg-error ".E0277." "" { target *-*-* } }
    offset_of!(Gamma, z); // { dg-error ".E0277." "" { target *-*-* } }
    offset_of!((u8, dyn Trait), 0); // ok
    offset_of!((u8, dyn Trait), 1); // { dg-error ".E0277." "" { target *-*-* } }
}

fn delta() {
    offset_of!(Delta<Alpha>, z); // { dg-error ".E0277." "" { target *-*-* } }
    offset_of!(Delta<Extern>, z); // { dg-error ".E0277." "" { target *-*-* } }
    offset_of!(Delta<dyn Trait>, z); // { dg-error ".E0277." "" { target *-*-* } }
}

fn generic_with_maybe_sized<T: ?Sized>() -> usize {
    offset_of!(Delta<T>, z) // { dg-error ".E0277." "" { target *-*-* } }
}

fn illformed_tuple() {
    offset_of!(([u8], u8), 1); // { dg-error ".E0277." "" { target *-*-* } }
}

