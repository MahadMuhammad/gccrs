// { dg-additional-options "-frust-edition= 2021" }

#![feature(unsized_locals)]
// { dg-warning "" "" { target *-*-* } .-1 }

async fn f() {}

async fn g(x: Box<dyn std::fmt::Display>) {
    let _x = *x;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    f().await;
}

fn main() {
    let _a = g(Box::new(5));
}

