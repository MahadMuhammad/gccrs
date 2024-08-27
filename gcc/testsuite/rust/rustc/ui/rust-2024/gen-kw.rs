//@ revisions: e2015 e2018
// { dg-additional-options "-frust-edition= 2018" }

#![deny(rust_2024_compatibility)]

fn gen() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }

fn main() {
    let gen = r#gen;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
}

macro_rules! t {
    () => { mod test { fn gen() {} } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
}

t!();

