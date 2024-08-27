//! This is a regression test to avoid an ICE in diagnostics code.
//! A typo in the compiler used to get the DefId of FnOnce, and
//! use it where an associated item was expected.

fn frob() -> impl Fn<P, Output = T> + '_ {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-error ".E0658." "" { target *-*-* } .-5 }

fn open_parent<'path>() {
    todo!()
}

fn main() {
    let old_path = frob("hello");
// { dg-error ".E0061." "" { target *-*-* } .-1 }

    open_parent(&old_path)
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

