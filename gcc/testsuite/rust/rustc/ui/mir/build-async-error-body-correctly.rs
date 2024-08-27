// { dg-additional-options "-frust-edition= 2021" }

async fn asyncfn() {
    let binding = match true {};
// { dg-error ".E0004." "" { target *-*-* } .-1 }
}

fn main() {}

