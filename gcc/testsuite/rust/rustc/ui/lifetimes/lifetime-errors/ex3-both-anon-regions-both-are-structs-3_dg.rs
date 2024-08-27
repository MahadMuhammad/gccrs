struct Ref<'a, 'b> {
    a: &'a u32,
    b: &'b u32,
}

fn foo(mut x: Ref) {
    x.a = x.b;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

