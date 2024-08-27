// { dg-additional-options "-frust-edition=2018" }

fn main() {
    let s = ::xcrate::S;
// { dg-error ".E0433." "" { target *-*-* } .-1 }
}

