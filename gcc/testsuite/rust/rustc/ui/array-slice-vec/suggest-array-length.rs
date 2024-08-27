//@ run-rustfix
#![allow(unused_variables, dead_code, non_upper_case_globals)]

fn main() {
    const Foo: [i32; _] = [1, 2, 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    const REF_FOO: &[u8; _] = &[1];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    let foo: [i32; _] = [1, 2, 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    let bar: [i32; _] = [0; 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    let ref_foo: &[i32; _] = &[1, 2, 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    let ref_bar: &[i32; _] = &[0; 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
    let multiple_ref_foo: &&[i32; _] = &&[1, 2, 3];
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
}

