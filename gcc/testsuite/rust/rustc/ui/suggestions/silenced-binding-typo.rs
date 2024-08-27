//@ run-rustfix
fn main() {
    let _x = 42; // { help "" "" { target *-*-* } }
    let _y = x; // { dg-error ".E0425." "" { target *-*-* } }
}

