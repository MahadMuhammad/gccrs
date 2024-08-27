// { dg-additional-options "-frust-edition= 2021" }

fn test<'s: 's>(s: &'s str) -> impl std::future::Future<Output = impl Sized> {
    async move { let _s = s; }
// { dg-error ".E0700." "" { target *-*-* } .-1 }
}

fn main() {}

