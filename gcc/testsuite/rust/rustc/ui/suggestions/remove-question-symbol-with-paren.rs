// https://github.com/rust-lang/rust/issues/114392

fn foo() -> Option<()> {
    let x = Some(());
    (x?)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

