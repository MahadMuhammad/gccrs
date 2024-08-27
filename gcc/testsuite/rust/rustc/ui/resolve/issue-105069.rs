use self::A::*;
use V; // { dg-error ".E0659." "" { target *-*-* } }
use self::B::*;
enum A {
    V
}
enum B {
    V
}

fn main() {}

