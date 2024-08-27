// Regression test for ICE #124848
// Tests that there is no ICE when a cast
// involves a type with error

use std::cell::Cell;

struct MyType<'a>(Cell<Option<&'unpinned mut MyType<'a>>>, Pin);
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }

fn main() {
    let mut unpinned = MyType(Cell::new(None));
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    let bad_addr = &unpinned as *const Cell<Option<&'a mut MyType<'a>>> as usize;
// { dg-error ".E0606." "" { target *-*-* } .-1 }
// { dg-error ".E0606." "" { target *-*-* } .-2 }
// { dg-error ".E0606." "" { target *-*-* } .-3 }
}

