// { dg-additional-options "-frust-edition=2021" }
// issue: rust-lang/rust#127555

pub trait Foo {
    fn bar<F>(&mut self, func: F) -> impl std::future::Future<Output = ()> + Send
    where
        F: FnMut();
}

struct Baz {}

impl Foo for Baz {
    async fn bar<F>(&mut self, _func: F) -> ()
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    where
        F: FnMut() + Send,
// { dg-error ".E0276." "" { target *-*-* } .-1 }
    {
        ()
    }
}

fn main() {}

