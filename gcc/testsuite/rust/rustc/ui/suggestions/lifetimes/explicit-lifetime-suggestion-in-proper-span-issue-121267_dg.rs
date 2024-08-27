fn main() {}

fn foo(_src: &crate::Foo) -> Option<i32> {
    todo!()
}
fn bar(src: &crate::Foo) -> impl Iterator<Item = i32> {
    [0].into_iter()
// { dg-error ".E0700." "" { target *-*-* } .-1 }
        .filter_map(|_| foo(src))
}

struct Foo<'a>(&'a str);

