// { dg-additional-options "-frust-edition=2018" }

#![deny(unreachable_code)]

async fn foo() {
    endless().await;
    println!("this is unreachable!");
// { dg-error "" "" { target *-*-* } .-1 }
}

async fn endless() -> ! {
    loop {}
}

fn main() { }

