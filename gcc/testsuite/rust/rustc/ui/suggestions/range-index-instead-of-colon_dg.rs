// { dg-additional-options "-frust-edition=2021" }

fn main() {
    &[1, 2, 3][1:2];
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

