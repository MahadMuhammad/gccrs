// Error reporting for where `for<T> T: Trait` doesn't hold

#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Trait {}

fn fail()
where
    for<T> T: Trait,
{}

fn auto_trait()
where
    for<T> T: Send,
{}

fn main() {
    fail();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    auto_trait();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

