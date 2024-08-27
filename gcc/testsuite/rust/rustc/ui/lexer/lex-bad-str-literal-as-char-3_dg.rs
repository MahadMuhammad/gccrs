//@ revisions: rust2015 rust2018 rust2021
// { dg-additional-options "-frust-edition=2018" }
// { dg-additional-options "-frust-edition=2021" }
fn main() {
    println!('hello world');
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

