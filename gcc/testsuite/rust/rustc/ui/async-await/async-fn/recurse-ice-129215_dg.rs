// { dg-additional-options "-frust-edition= 2021" }

async fn a() {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    a() // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

