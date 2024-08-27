// { dg-additional-options "-frust-edition=2021" }
// gate-test-async_for_loop

#![feature(async_iter_from_iter, async_iterator)]

fn f() {
    let _ = async {
        for await _i in core::async_iter::from_iter(0..3) {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        }
    };
}

#[cfg(FALSE)]
fn g() {
    let _ = async {
        for await _i in core::async_iter::from_iter(0..3) {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
        }
    };
}

fn main() {}

