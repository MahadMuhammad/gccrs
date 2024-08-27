// { dg-additional-options "-frust-edition=2021" }
// gate-test-coroutine_clone
// Verifies that feature(coroutine_clone) doesn't allow async blocks to be cloned/copied.

#![feature(coroutines, coroutine_clone)]

use std::future::ready;

struct NonClone;

fn main() {
    let inner_non_clone = async {
        let non_clone = NonClone;
        let () = ready(()).await;
        drop(non_clone);
    };
    check_copy(&inner_non_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&inner_non_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let non_clone = NonClone;
    let outer_non_clone = async move {
        drop(non_clone);
    };
    check_copy(&outer_non_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&outer_non_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let maybe_copy_clone = async move {};
    check_copy(&maybe_copy_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&maybe_copy_clone);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let inner_non_clone_fn = the_inner_non_clone_fn();
    check_copy(&inner_non_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&inner_non_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let outer_non_clone_fn = the_outer_non_clone_fn(NonClone);
    check_copy(&outer_non_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&outer_non_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let maybe_copy_clone_fn = the_maybe_copy_clone_fn();
    check_copy(&maybe_copy_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_clone(&maybe_copy_clone_fn);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

async fn the_inner_non_clone_fn() {
    let non_clone = NonClone;
    let () = ready(()).await;
    drop(non_clone);
}

async fn the_outer_non_clone_fn(non_clone: NonClone) {
    let () = ready(()).await;
    drop(non_clone);
}

async fn the_maybe_copy_clone_fn() {}

fn check_copy<T: Copy>(_x: &T) {}
fn check_clone<T: Clone>(_x: &T) {}

