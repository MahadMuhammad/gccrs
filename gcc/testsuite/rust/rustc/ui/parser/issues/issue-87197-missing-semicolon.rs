//@ run-rustfix
// Parser should know when a semicolon is missing.
// https://github.com/rust-lang/rust/issues/87197

fn main() {
    let x = 100 // { dg-error "" "" { target *-*-* } }
    println!("{}", x) // { dg-error "" "" { target *-*-* } }
    let y = 200 // { dg-error "" "" { target *-*-* } }
    println!("{}", y);
}

