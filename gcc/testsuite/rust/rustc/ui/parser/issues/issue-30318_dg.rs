//@ run-rustfix
#![allow(unused)]
fn foo() { }

//! Misplaced comment...
// { dg-error ".E0753." "" { target *-*-* } .-1 }
fn bar() { } // { dg-note "" "" { target *-*-* } }

#![test] // { dg-error "" "" { target *-*-* } }
fn baz() { } // { dg-note "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }

/*! Misplaced comment... */
// { dg-error ".E0753." "" { target *-*-* } .-1 }
fn bat() { } // { dg-note "" "" { target *-*-* } }

fn main() { }

//! Misplaced comment...
// { dg-error ".E0753." "" { target *-*-* } .-1 }
// { dg-note ".E0753." "" { target *-*-* } .-2 }
// { dg-note ".E0753." "" { target *-*-* } .-3 }
/*! Misplaced comment... */
// { dg-error ".E0753." "" { target *-*-* } .-1 }
// { dg-note ".E0753." "" { target *-*-* } .-2 }
// { dg-error ".E0753." "" { target *-*-* } .-3 }
// { dg-note ".E0753." "" { target *-*-* } .-4 }

