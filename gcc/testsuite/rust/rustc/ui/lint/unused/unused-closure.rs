// Test that closures and coroutines are "must use" types.
// { dg-additional-options "-frust-edition=2018" }

#![feature(async_closure)]
#![feature(coroutines, stmt_expr_attributes)]
#![deny(unused_must_use)]

fn unused() {
    || { // { dg-error "" "" { target *-*-* } }
        println!("Hello!");
    };

    async {};    // { dg-error "" "" { target *-*-* } }
    || async {}; // { dg-error "" "" { target *-*-* } }
    async || {}; // { dg-error "" "" { target *-*-* } }


    [Box::new([|| {}; 10]); 1]; // { dg-error "" "" { target *-*-* } }

    vec![|| "a"].pop().unwrap(); // { dg-error "" "" { target *-*-* } }

    let b = false;
        || true; // { dg-error "" "" { target *-*-* } }
    println!("{}", b);
}

fn ignored() {
    let _ = || {};
    let _ = #[coroutine] || yield 42;
}

fn main() {
    unused();
    ignored();
}

