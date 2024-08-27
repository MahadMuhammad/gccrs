//@ run-rustfix
fn main() {
    println!(' 1 + 1'); // { dg-error "" "" { target *-*-* } }
}

