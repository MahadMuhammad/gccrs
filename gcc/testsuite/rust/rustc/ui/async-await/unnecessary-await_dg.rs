// { dg-additional-options "-frust-edition=2018" }

async fn foo () { }
fn bar() -> impl std::future::Future { async {} }
fn boo() {}

async fn baz() -> std::io::Result<()> {
    foo().await;
    boo().await; // { dg-error ".E0277." "" { target *-*-* } }
    bar().await;
    std::io::Result::Ok(())
}

macro_rules! e {
    () => {
        ()
    };
}

macro_rules! f {
    ($expr:expr) => {
        $expr.await
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    };
}

async fn with_macros() {
    e!().await;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    f!(());
}

// Regression test for issue #117014.
async fn desugaring_span_ctxt() {
    for x in [] {}.await // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

