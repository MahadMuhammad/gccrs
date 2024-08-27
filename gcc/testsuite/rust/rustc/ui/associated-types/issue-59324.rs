trait NotFoo {}

pub trait Foo: NotFoo {
    type OnlyFoo;
}

pub trait Service {
    type AssocType;
}

pub trait ThriftService<Bug: NotFoo>:
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    Service<AssocType = <Bug as Foo>::OnlyFoo>
{
    fn get_service(
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        &self,
    ) -> Self::AssocType;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn with_factory<H>(factory: dyn ThriftService<()>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }

fn main() {}

