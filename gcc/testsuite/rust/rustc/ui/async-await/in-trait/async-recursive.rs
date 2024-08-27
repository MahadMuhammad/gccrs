// { dg-additional-options "-frust-edition= 2021" }

trait MyTrait {
    async fn foo_recursive(&self, n: usize) -> i32;
}

impl MyTrait for i32 {
    async fn foo_recursive(&self, n: usize) -> i32 {
// { dg-error ".E0733." "" { target *-*-* } .-1 }
        if n > 0 {
            self.foo_recursive(n - 1).await
        } else {
            *self
        }
    }
}

fn main() {}

