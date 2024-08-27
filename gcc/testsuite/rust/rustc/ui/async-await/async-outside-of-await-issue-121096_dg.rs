// { dg-additional-options "-frust-edition=2021" }

fn main() {
    async {
        use std::ops::Add;
        let _ = 1.add(3);
    }.await
// { dg-error ".E0728." "" { target *-*-* } .-1 }
}

