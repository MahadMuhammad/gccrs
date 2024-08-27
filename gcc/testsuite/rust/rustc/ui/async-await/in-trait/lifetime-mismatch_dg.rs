// { dg-additional-options "-frust-edition=2021" }


trait MyTrait {
    async fn foo<'a>(&self);
    async fn bar(&self);
}

impl MyTrait for i32 {
    async fn foo(&self) {}
// { dg-error ".E0195." "" { target *-*-* } .-1 }

    async fn bar(&self) {
        self.foo();
    }
}

fn main() {}

