#![feature(unsized_fn_params)]

fn main() {
    // Simple coercion
    let f: fn([u8]) = |_result| {};
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let slice: Box<[u8]> = Box::new([1; 8]);
    f(*slice);
}

