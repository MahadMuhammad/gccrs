// { dg-additional-options "-frust-edition=2018" }
#![feature(must_not_suspend)]
#![deny(must_not_suspend)]

#[must_not_suspend]
struct No {}

async fn shushspend() {}

async fn wheeee<T>(t: T) {
    shushspend().await;
    drop(t);
}

async fn yes() {
    let no = No {};
// { dg-error "" "" { target *-*-* } .-1 }
    wheeee(&no).await;
    drop(no);
}

fn main() {
}

