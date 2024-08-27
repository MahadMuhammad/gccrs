#![deny(unreachable_code)]

fn main() {
    let x = || -> ! { panic!() };
    x();
    println!("Foo bar"); // { dg-error "" "" { target *-*-* } }
}

