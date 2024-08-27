pub fn bad(x: &mut bool) {
    if true
    *x = true {}
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

pub fn bad2(x: &mut bool) {
    let y: bool;
    y = true
    *x = true;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

fn main() {}

