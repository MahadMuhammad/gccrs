//@ compile-flags: -Zverbose-internals

fn foo(_: i32, _: i32) {}

fn needs_ptr(_: fn(i32, u32)) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

fn main() {
    needs_ptr(foo);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
}

