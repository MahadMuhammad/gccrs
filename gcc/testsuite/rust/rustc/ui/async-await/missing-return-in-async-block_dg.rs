//@ run-rustfix
// { dg-additional-options "-frust-edition=2021" }
use std::future::Future;
use std::pin::Pin;
pub struct S;
pub fn foo() {
    let _ = Box::pin(async move {
        if true {
            Ok(S) // { dg-error ".E0308." "" { target *-*-* } }
        }
        Err(())
    });
}
pub fn bar() -> Pin<Box<dyn Future<Output = Result<S, ()>> + 'static>> {
    Box::pin(async move {
        if true {
            Ok(S) // { dg-error ".E0308." "" { target *-*-* } }
        }
        Err(())
    })
}
fn main() {}

