#![feature(extern_types)]

extern "Rust" {
    type OpaqueListContents;
}

pub struct ListS<T> {
// { dg-note "" "" { target *-*-* } .-1 }
    len: usize,
    data: [T; 0],
    opaque: OpaqueListContents,
}

pub struct Interned<'a, T>(&'a T);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

impl<'a, T> Clone for Interned<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T> Copy for Interned<'a, T> {}

pub struct List<'tcx, T>(Interned<'tcx, ListS<T>>);
// { dg-note ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }

impl<'tcx, T> Clone for List<'tcx, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'tcx, T> Copy for List<'tcx, T> {}
// { dg-error ".E0204." "" { target *-*-* } .-1 }

fn assert_is_copy<T: Copy>() {}

fn main() {
    assert_is_copy::<List<'static, ()>>();
}

