// ICE Unexpected unsized type tail: &ReStatic [u8]
// issue: rust-lang/rust#122488
use std::ops::Deref;

struct ArenaSet<U: Deref, V: ?Sized = <U as Deref>::Target>(V, U);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

const DATA: *const ArenaSet<Vec<u8>> = std::ptr::null_mut();

pub fn main() {}

