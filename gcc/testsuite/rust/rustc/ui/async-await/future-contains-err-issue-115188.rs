// { dg-additional-options "-frust-edition= 2021" }

// Makes sure we don't spew a bunch of unrelated opaque errors when the reason
// for this error is just a missing struct field in `foo`.

async fn foo() {
    let y = Wrapper { };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
}

struct Wrapper<T> { t: T }

fn is_send<T: Send>(_: T) {}

fn main() {
    is_send(foo());
}

