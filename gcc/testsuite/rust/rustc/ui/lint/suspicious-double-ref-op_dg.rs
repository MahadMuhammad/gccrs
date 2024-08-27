#![deny(suspicious_double_ref_op, noop_method_call)]

use std::borrow::Borrow;
use std::ops::Deref;

struct PlainType<T>(T);

#[derive(Clone)]
struct CloneType<T>(T);

pub fn clone_on_double_ref() {
    let x = vec![1];
    let y = &&x;
    let z: &Vec<_> = y.clone();
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{:p} {:p}", *y, z);
}

use std::sync::LazyLock;

pub static STRS: LazyLock<&str> = LazyLock::new(|| "First");

// https://github.com/rust-lang/rust-clippy/issues/9272
fn rust_clippy_issue_9272() {
    let str = STRS.clone();
    println!("{str}")
}

fn main() {
    let clone_type_ref = &&CloneType(1u32);
    let clone_type_ref_clone: &CloneType<u32> = clone_type_ref.clone();
// { dg-error "" "" { target *-*-* } .-1 }

    let non_deref_type = &&PlainType(1u32);
    let non_deref_type_deref: &PlainType<u32> = non_deref_type.deref();
// { dg-error "" "" { target *-*-* } .-1 }

    let xs = ["a", "b", "c"];
    let _v: Vec<&str> = xs.iter().map(|x| x.clone()).collect(); // could use `*x` instead
// { dg-error "" "" { target *-*-* } .-1 }
}

