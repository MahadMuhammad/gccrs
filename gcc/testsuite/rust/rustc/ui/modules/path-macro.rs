macro_rules! foo {
    () => {"bar.rs"};
}

#[path = foo!()] // { dg-error "" "" { target *-*-* } }
mod abc;

fn main() {}

