// { dg-additional-options "-frust-edition=2018" }

struct Xyz {
    a: u64,
}

trait Foo {}

impl Xyz {
    async fn do_sth<'a>(
        foo: &dyn Foo, bar: &'a dyn Foo
    ) -> &dyn Foo // { dg-error ".E0106." "" { target *-*-* } }
    {
// { dg-error ".E0621." "" { target *-*-* } .-1 }
        foo
    }
}

fn main() {}

