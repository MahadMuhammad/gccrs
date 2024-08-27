#![feature(unsized_fn_params)]

fn main() {
    // CoerceMany "LUB"
    let f = if true { |_a| {} } else { |_b| {} };
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }

    let slice: Box<[u8]> = Box::new([1; 8]);
    f(*slice);
}

