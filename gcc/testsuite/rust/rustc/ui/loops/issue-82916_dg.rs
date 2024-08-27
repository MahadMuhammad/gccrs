struct S(i32);

fn foo(x: Vec<S>) {
    for y in x {

    }
    let z = x; // { dg-error ".E0382." "" { target *-*-* } }
}

fn main() {}

