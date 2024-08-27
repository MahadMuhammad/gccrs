// { dg-additional-options "-frust-edition= 2021" }

trait MyTrait {
    async fn foo(self) -> (Self, i32);
}

impl MyTrait for xyz::T { // { dg-error ".E0433." "" { target *-*-* } }
    async fn foo(self, key: i32) -> (u32, i32) {
        (self, key)
    }
}

fn main() {}

