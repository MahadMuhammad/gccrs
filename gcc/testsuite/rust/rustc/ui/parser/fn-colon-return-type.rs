fn foo(x: i32): i32 {
// { dg-error "" "" { target *-*-* } .-1 }
    x
}

fn main() {}

