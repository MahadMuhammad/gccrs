//@ run-pass
//@ revisions: e2015 e2018 e2021 e2024

// { dg-additional-options "-frust-edition=2018" }
// { dg-additional-options "-frust-edition=2021" }
// { dg-additional-options "-frust-edition=2024" }

//@[e2024] compile-flags: -Zunstable-options

fn main() {
    println!("hello");
}

