#![feature(unsized_fn_params)]

fn main() {
    let f: fn([u8]) = |_| {};
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let slice: Box<[u8]> = Box::new([1; 8]);

    f(*slice);
}

