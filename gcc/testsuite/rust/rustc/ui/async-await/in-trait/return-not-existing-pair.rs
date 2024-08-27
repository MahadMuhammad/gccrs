// { dg-additional-options "-frust-edition=2021" }


trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a ConnImpl, &'b T);
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl<'a, 'b, T, U> MyTrait<T> for U {
// { dg-error ".E0726." "" { target *-*-* } .-1 }
    async fn foo(_: T) -> (&'a U, &'b T) {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

