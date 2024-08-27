/*
 * We don't infer `T: 'static` outlives relationships.
 */

struct Foo<U> {
    bar: Bar<U> // { dg-error ".E0310." "" { target *-*-* } }
}
struct Bar<T: 'static> {
    x: T,
}

fn main() {}

