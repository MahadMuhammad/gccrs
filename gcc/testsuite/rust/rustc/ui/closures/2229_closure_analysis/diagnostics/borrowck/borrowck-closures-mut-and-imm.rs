// { dg-additional-options "-frust-edition=2021" }



// Tests that two closures cannot simultaneously have mutable
// and immutable access to the variable. Issue #6801.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn a() {
    let mut p = Point {x: 3, y:4};
    let c2 = || p.y * 5;
    let c1 = || {
// { dg-error ".E0502." "" { target *-*-* } .-1 }
        dbg!(&p);
        p.x = 4;
    };
    drop(c2);
}

fn main() {
}

