// Checks that certain traits for which we don't want to suggest borrowing
// are blacklisted and don't cause the suggestion to be issued.

#![feature(coroutines)]

fn f_copy<T: Copy>(t: T) {}
fn f_clone<T: Clone>(t: T) {}
fn f_unpin<T: Unpin>(t: T) {}
fn f_sized<T: Sized>(t: T) {}
fn f_send<T: Send>(t: T) {}

struct S;

fn main() {
    f_copy("".to_string()); // { dg-error ".E0277." "" { target *-*-* } }
    f_clone(S); // { dg-error ".E0277." "" { target *-*-* } }
    f_unpin(#[coroutine] static || { yield; });
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let cl = || ();
    let ref_cl: &dyn Fn() -> () = &cl;
    f_sized(*ref_cl);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    use std::rc::Rc;
    let rc = Rc::new(0);
    f_send(rc); // { dg-error ".E0277." "" { target *-*-* } }
}

