fn main() {
    g((), ());
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

pub fn g(a1: (), a2: bool, a3: bool, a4: bool, a5: bool, a6: ()) -> () {}

