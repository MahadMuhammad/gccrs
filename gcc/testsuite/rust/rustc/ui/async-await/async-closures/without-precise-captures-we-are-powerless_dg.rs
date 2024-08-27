// { dg-additional-options "-frust-edition= 2018" }

// This is `no-borrow-from-env.rs`, but under edition 2018 we still want to make
// sure that we don't ICE or anything, even if precise closure captures means
// that we can't actually borrowck successfully.

#![feature(async_closure)]

fn outlives<'a>(_: impl Sized + 'a) {}

async fn call_once(f: impl async FnOnce()) {
    f().await;
}

fn simple<'a>(x: &'a i32) {
    let c = async || { println!("{}", *x); }; // { dg-error ".E0597." "" { target *-*-* } }
    outlives::<'a>(c());
    outlives::<'a>(call_once(c));

    let c = async move || { println!("{}", *x); };
    outlives::<'a>(c()); // { dg-error ".E0597." "" { target *-*-* } }
    outlives::<'a>(call_once(c));
}

struct S<'a>(&'a i32);

fn through_field<'a>(x: S<'a>) {
    let c = async || { println!("{}", *x.0); }; // { dg-error ".E0597." "" { target *-*-* } }
    outlives::<'a>(c());
    outlives::<'a>(call_once(c));

    let c = async move || { println!("{}", *x.0); }; // { dg-error ".E0505." "" { target *-*-* } }
    outlives::<'a>(c()); // { dg-error ".E0597." "" { target *-*-* } }
    outlives::<'a>(call_once(c)); // { dg-error ".E0505." "" { target *-*-* } }
}

fn through_field_and_ref<'a>(x: &S<'a>) {
    let c = async || { println!("{}", *x.0); }; // { dg-error ".E0597." "" { target *-*-* } }
    outlives::<'a>(c());
    outlives::<'a>(call_once(c)); // { dg-error ".E0621." "" { target *-*-* } }

    let c = async move || { println!("{}", *x.0); };
    outlives::<'a>(c()); // { dg-error ".E0597." "" { target *-*-* } }
    // outlives::<'a>(call_once(c)); // FIXME(async_closures): Figure out why this fails
}

fn main() {}

