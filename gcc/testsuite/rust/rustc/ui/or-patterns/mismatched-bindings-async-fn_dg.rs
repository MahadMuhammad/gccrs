// Regression test for #71297
// { dg-additional-options "-frust-edition=2018" }

async fn a((x | s): String) {}
// { dg-error ".E0408." "" { target *-*-* } .-1 }
// { dg-error ".E0408." "" { target *-*-* } .-2 }

async fn b() {
    let (x | s) = String::new();
// { dg-error ".E0408." "" { target *-*-* } .-1 }
// { dg-error ".E0408." "" { target *-*-* } .-2 }
}

fn main() {}

