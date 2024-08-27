// { dg-additional-options "-frust-edition=2018" }

async fn fun() {
    [1; ().await];
// { dg-error ".E0728." "" { target *-*-* } .-1 }
}

fn main() {}

