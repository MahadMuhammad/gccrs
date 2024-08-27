fn g(f: for<'a> fn(fn(&str, &'a str))) -> bool {
    f
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

