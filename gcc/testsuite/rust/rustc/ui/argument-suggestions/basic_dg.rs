// Some basic "obvious" cases for the heuristic error messages added for #65853
// One for each of the detected cases

enum E { X, Y }
enum F { X2, Y2 }
struct G {}
struct H {}
struct X {}
struct Y {}
struct Z {}


fn invalid(_i: u32) {}
fn extra() {}
fn missing(_i: u32) {}
fn swapped(_i: u32, _s: &str) {}
fn permuted(_x: X, _y: Y, _z: Z) {}

fn main() {
    invalid(1.0); // { dg-error ".E0308." "" { target *-*-* } }
    extra(""); // { dg-error ".E0061." "" { target *-*-* } }
    missing(); // { dg-error ".E0061." "" { target *-*-* } }
    swapped("", 1); // { dg-error ".E0308." "" { target *-*-* } }
    permuted(Y {}, Z {}, X {}); // { dg-error ".E0308." "" { target *-*-* } }

    let closure = |x| x;
    closure(); // { dg-error ".E0057." "" { target *-*-* } }
}

