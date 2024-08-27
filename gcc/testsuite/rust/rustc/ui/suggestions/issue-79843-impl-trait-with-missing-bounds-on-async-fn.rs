// Regression test: if we suggest replacing an `impl Trait` argument to an async
// fn with a named type parameter in order to add bounds, the suggested function
// signature should be well-formed.
//
// { dg-additional-options "-frust-edition=2018" }

trait Foo {
    type Bar;
    fn bar(&self) -> Self::Bar;
}

async fn run(_: &(), foo: impl Foo) -> std::io::Result<()> {
    let bar = foo.bar();
    assert_is_send(&bar);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    Ok(())
}

// Test our handling of cases where there is a generic parameter list in the
// source, but only synthetic generic parameters
async fn run2< >(_: &(), foo: impl Foo) -> std::io::Result<()> {
    let bar = foo.bar();
    assert_is_send(&bar);
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    Ok(())
}

fn assert_is_send<T: Send>(_: &T) {}

fn main() {}

