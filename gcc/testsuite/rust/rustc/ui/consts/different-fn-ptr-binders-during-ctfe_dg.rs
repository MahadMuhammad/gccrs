const fn cmp(x: fn(&'static ()), y: for<'a> fn(&'a ())) -> bool {
    x == y
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

