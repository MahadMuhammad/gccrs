//@ compile-flags: -F unused -A unused

fn main() {
    let x = 1;
// { dg-error "" "" { target *-*-* } .-1 }
}

