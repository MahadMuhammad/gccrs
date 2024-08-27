// issue: rust-lang/rust#127868

fn main() {
    let a = [[[[[[[[[[[[[[[[[[[[1, {, (, [,;
} // { dg-error "" "" { target *-*-* } }
// { dg-error "" "" { target *-*-* } }

