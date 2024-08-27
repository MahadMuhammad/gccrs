struct Ref<'a, 'b> {
    a: &'a u32,
    b: &'b u32,
}

fn foo(mut x: Ref, y: Ref) {
    x.b = y.b;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

