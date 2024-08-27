//FIXME(compiler-errors): This fixup should suggest the full box path, not just `Box`

struct Box<T> {
    t: T,
}

struct Foo {
// { dg-error ".E0072." "" { target *-*-* } .-1 }
    inner: Foo,
}

fn main() {}

