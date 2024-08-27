use std::cell::RefCell;
use std::mem::ManuallyDrop;

union U1 {
    a: u8,
}

union U2 {
    a: ManuallyDrop<String>,
}

union U3<T> {
    a: ManuallyDrop<T>,
}

union U4<T: Copy> {
    a: T,
}

union URef {
    p: &'static mut i32,
}

union URefCell {
    // field that does not drop but is not `Copy`, either
    a: (ManuallyDrop<RefCell<i32>>, i32),
}

fn deref_union_field(mut u: URef) {
    // Not an assignment but an access to the union field!
    *(u.p) = 13; // { dg-error ".E0133." "" { target *-*-* } }
}

fn assign_noncopy_union_field(mut u: URefCell) {
    u.a = (ManuallyDrop::new(RefCell::new(0)), 1); // OK (assignment does not drop)
    u.a.0 = ManuallyDrop::new(RefCell::new(0)); // OK (assignment does not drop)
    u.a.1 = 1; // OK
}

fn generic_noncopy<T: Default>() {
    let mut u3 = U3 { a: ManuallyDrop::new(T::default()) };
    u3.a = ManuallyDrop::new(T::default()); // OK (assignment does not drop)
    *u3.a = T::default(); // { dg-error ".E0133." "" { target *-*-* } }
}

fn generic_copy<T: Copy + Default>() {
    let mut u3 = U3 { a: ManuallyDrop::new(T::default()) };
    u3.a = ManuallyDrop::new(T::default()); // OK
    *u3.a = T::default(); // { dg-error ".E0133." "" { target *-*-* } }

    let mut u4 = U4 { a: T::default() };
    u4.a = T::default(); // OK
}

fn main() {
    let mut u1 = U1 { a: 10 }; // OK
    let a = u1.a; // { dg-error ".E0133." "" { target *-*-* } }
    u1.a = 11; // OK

    let U1 { a } = u1; // { dg-error ".E0133." "" { target *-*-* } }
    if let U1 { a: 12 } = u1 {} // { dg-error ".E0133." "" { target *-*-* } }
    if let Some(U1 { a: 13 }) = Some(u1) {} // { dg-error ".E0133." "" { target *-*-* } }
    // let U1 { .. } = u1; // OK

    let mut u2 = U2 { a: ManuallyDrop::new(String::from("old")) }; // OK
    u2.a = ManuallyDrop::new(String::from("new")); // OK (assignment does not drop)
    *u2.a = String::from("new"); // { dg-error ".E0133." "" { target *-*-* } }

    let mut u3 = U3 { a: ManuallyDrop::new(0) }; // OK
    u3.a = ManuallyDrop::new(1); // OK
    *u3.a = 1; // { dg-error ".E0133." "" { target *-*-* } }

    let mut u3 = U3 { a: ManuallyDrop::new(String::from("old")) }; // OK
    u3.a = ManuallyDrop::new(String::from("new")); // OK (assignment does not drop)
    *u3.a = String::from("new"); // { dg-error ".E0133." "" { target *-*-* } }
}

