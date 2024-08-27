// from rfc2005 test suite



pub fn main() {
    let Some(x) = &Some(3) else {
        panic!();
    };
    *x += 1; // { dg-error ".E0594." "" { target *-*-* } }
}

