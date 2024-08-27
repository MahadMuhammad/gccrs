// { dg-additional-options "-frust-edition=2021" }
#![feature(negative_impls)]
#![allow(unused)]

fn main() {
    gimme_send(foo());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
}

fn gimme_send<T: Send>(t: T) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    drop(t);
}

struct NotSend {}

impl Drop for NotSend {
    fn drop(&mut self) {}
}

impl !Send for NotSend {}

async fn foo() {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    let mut x = (NotSend {},);
    drop(x.0);
    x.0 = NotSend {};
    bar().await;
}

async fn bar() {}

