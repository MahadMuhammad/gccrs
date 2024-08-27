// { dg-additional-options "-frust-edition=2021" }
// Check what happens when a const async fn is in the main function (#102796)

fn main() {
    const async fn a() {}
// { dg-error "" "" { target *-*-* } .-1 }
}

