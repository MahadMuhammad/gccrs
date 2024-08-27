// { dg-additional-options "-frust-edition=2021" }


trait Foo {
    async fn foo(&self);
}

fn main() {
    let x: &dyn Foo = todo!();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

