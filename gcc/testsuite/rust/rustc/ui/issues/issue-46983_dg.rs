fn foo(x: &u32) -> &'static u32 {
    &*x
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

