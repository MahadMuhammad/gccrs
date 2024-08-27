//@ run-rustfix

fn main() {
    let x == 2; // { dg-error "" "" { target *-*-* } }
    println!("x: {}", x)
}

