//@ run-rustfix
fn main() {}
fn _foo() -> bool {
    &  // { dg-error ".E0308." "" { target *-*-* } }
    mut
    if true { true } else { false }
}

fn _bar() -> bool {
    &  // { dg-error ".E0308." "" { target *-*-* } }
    mut if true { true } else { false }
}

fn _baz() -> bool {
    & mut // { dg-error ".E0308." "" { target *-*-* } }
    if true { true } else { false }
}

