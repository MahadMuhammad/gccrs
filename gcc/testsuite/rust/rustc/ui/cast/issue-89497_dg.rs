// Regression test for issue #89497.

//@ run-rustfix

fn main() {
    let pointer: usize = &1_i32 as *const i32 as usize;
    let _reference: &'static i32 = unsafe { pointer as *const i32 as &'static i32 };
// { dg-error ".E0605." "" { target *-*-* } .-1 }
// { help ".E0605." "" { target *-*-* } .-2 }
}

