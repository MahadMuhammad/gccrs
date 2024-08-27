// { dg-additional-options "-frust-edition= 2021" }

trait MyTrait<T> {
    async fn foo_recursive(&self, n: usize) -> T;
}

impl<T> MyTrait<T> for T where T: Copy {
    async fn foo_recursive(&self, n: usize) -> T {
// { dg-error ".E0733." "" { target *-*-* } .-1 }
        if n > 0 {
            self.foo_recursive(n - 1).await
        } else {
            *self
        }
    }
}

fn main() {}

