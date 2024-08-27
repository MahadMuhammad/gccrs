// { dg-additional-options "-frust-edition=2021" }

// Check that precise paths are being reported back in the error message.

struct Y {
    y: X
}

struct X {
    a: u32,
    b: u32,
}

fn main() {
    let mut x = Y { y: X { a: 5, b: 0 } };
    let hello = || {
        x.y.a += 1;
    };

    let b = hello;
    let c = hello; // { dg-error ".E0382." "" { target *-*-* } }
}

