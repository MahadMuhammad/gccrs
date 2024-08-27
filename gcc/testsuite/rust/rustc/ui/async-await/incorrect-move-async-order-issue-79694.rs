//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }

// Regression test for issue 79694

fn main() {
    let _ = move async { }; // { dg-error "" "" { target *-*-* } }
}

