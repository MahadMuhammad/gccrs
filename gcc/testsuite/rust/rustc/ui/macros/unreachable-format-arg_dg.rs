//@ run-fail
//@ ignore-emscripten no processes

//@ revisions: edition_2015 edition_2021
// { dg-additional-options "-frust-edition=2015" }
// { dg-additional-options "-frust-edition=2021" }
//@ [edition_2015]error-pattern:internal error: entered unreachable code: x is {x}
//@ [edition_2021]error-pattern:internal error: entered unreachable code: x is 5

#![allow(non_fmt_panics)]

fn main() {
    let x = 5;
    unreachable!("x is {x}");
}

