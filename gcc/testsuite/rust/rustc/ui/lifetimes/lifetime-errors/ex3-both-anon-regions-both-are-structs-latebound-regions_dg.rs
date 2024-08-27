struct Ref<'a> {
    x: &'a u32,
}

fn foo<'a, 'b>(mut x: Vec<Ref<'a>>, y: Ref<'b>) {
    x.push(y);
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

