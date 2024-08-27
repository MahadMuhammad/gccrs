//@ check-pass
// { dg-additional-options "-frust-edition=2021" }

#[track_caller]
fn f() {
    let _ = async {};
}

fn main() {
    f();
}

