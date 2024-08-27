// { dg-additional-options "-frust-edition=2018" }

async fn free(); // { dg-error "" "" { target *-*-* } }

struct A;
impl A {
    async fn inherent(); // { dg-error "" "" { target *-*-* } }
}

trait B {
    async fn associated();
}
impl B for A {
    async fn associated(); // { dg-error "" "" { target *-*-* } }
}

fn main() {}

