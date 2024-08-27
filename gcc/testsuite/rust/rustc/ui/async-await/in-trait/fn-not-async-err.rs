// { dg-additional-options "-frust-edition= 2021" }

trait MyTrait {
    async fn foo(&self) -> i32;
}

impl MyTrait for i32 {
    fn foo(&self) -> i32 {
// { dg-error "" "" { target *-*-* } .-1 }
        *self
    }
}

fn main() {}

