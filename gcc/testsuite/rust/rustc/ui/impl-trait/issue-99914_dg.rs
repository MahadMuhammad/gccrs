// { dg-additional-options "-frust-edition=2021" }

fn main() {}

struct Error;
struct Okay;

fn foo(t: Result<Okay, Error>) {
    t.and_then(|t| -> _ { bar(t) });
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

async fn bar(t: Okay) {}

