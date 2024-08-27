// { dg-additional-options "-frust-edition=2021" }

struct S(String, String);

fn expect_fn<F: Fn()>(_f: F) {}

fn main() {
    let s = S(format!("s"), format!("s"));
    let c = || { // { dg-error ".E0525." "" { target *-*-* } }
        let s = s.1;
    };
    expect_fn(c);
}

