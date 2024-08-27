// { dg-additional-options "-frust-edition=2018" }
#![deny(unused_must_use)]


#[must_use]
async fn foo() -> i32 {
    1
}

#[must_use]
fn bar() -> impl std::future::Future<Output=i32> {
    async {
        42
    }
}

async fn baz() -> i32 {
    0
}

struct Wowee {}

impl Wowee {
    #[must_use]
    async fn test_method() -> i32 {
        1
    }
}

async fn test() {
    foo(); // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    foo().await; // { dg-error "" "" { target *-*-* } }
    bar(); // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
    bar().await; // ok, it's not an async fn
    baz(); // { dg-error "" "" { target *-*-* } }
    baz().await; // ok
}

/* FIXME(guswynn) update this test when async-fn-in-traits works
trait Doer {
    #[must_use]
    async fn test_trait_method() -> i32;
    WARNING must_use
    async fn test_other_trait() -> i32;
}

impl Doer for Wowee {
    async fn test_trait_method() -> i32 {
        1
    }
    #[must_use]
    async fn test_other_trait() -> i32 {
        WARNING must_use
        1
    }
}
*/

fn main() {
}

