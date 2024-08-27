//@ run-rustfix
#![allow(unused)]

struct S {
    f: String,
}

fn main() {
    let _moved @ _from = String::from("foo"); // { dg-error ".E0382." "" { target *-*-* } }
    let _moved @ ref _from = String::from("foo"); // { dg-error "" "" { target *-*-* } }
    let ref _moved @ _from = String::from("foo"); // { dg-error ".E0382." "" { target *-*-* } }
// { dg-error ".E0382." "" { target *-*-* } .-1 }
    let ref _moved @ ref _from = String::from("foo"); // ok
    let _moved @ S { f } = S { f: String::from("foo") }; // { dg-error ".E0382." "" { target *-*-* } }
    let ref _moved @ S { f } = S { f: String::from("foo") }; // { dg-error ".E0382." "" { target *-*-* } }
// { dg-error ".E0382." "" { target *-*-* } .-1 }
    let ref _moved @ S { ref f } = S { f: String::from("foo") }; // ok
    let _moved @ S { ref f } = S { f: String::from("foo") }; // { dg-error "" "" { target *-*-* } }
}

