// This is a separate test from `issue-66693.rs` because array lengths are evaluated
// in a separate stage before `const`s and `statics` and so the error below is hit and
// the compiler exits before generating errors for the others.

fn main() {
    let _ = [0i32; panic!(2f32)];
// { dg-error "" "" { target *-*-* } .-1 }

    // ensure that conforming panics are handled correctly
    let _ = [false; panic!()];
// { dg-error ".E0080." "" { target *-*-* } .-1 }

    // typechecking halts before getting to this one
    let _ = ['a', panic!("panic in array len")];
}

