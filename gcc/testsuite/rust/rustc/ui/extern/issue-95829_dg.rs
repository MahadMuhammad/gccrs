// { dg-additional-options "-frust-edition=2018" }

extern {
    async fn L() { // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } .-1 }
        async fn M() {}
    }
}

fn main() {}

