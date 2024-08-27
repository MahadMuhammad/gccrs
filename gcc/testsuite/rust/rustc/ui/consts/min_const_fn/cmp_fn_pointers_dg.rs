const fn cmp(x: fn(), y: fn()) -> bool {
    unsafe { x == y }
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

