// { dg-additional-options "-frust-edition=2021" }


trait MyTrait {
    async fn bar(&abc self);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

impl MyTrait for () {
    async fn bar(&self) {}
}

fn main() {}

