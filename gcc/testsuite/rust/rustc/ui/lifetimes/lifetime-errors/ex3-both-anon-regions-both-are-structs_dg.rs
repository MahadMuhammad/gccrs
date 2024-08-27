struct Ref<'a> {
    x: &'a u32,
}

fn foo(mut x: Vec<Ref>, y: Ref) {
    x.push(y);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

