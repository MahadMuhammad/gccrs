//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

trait A {
    type B<'b>;
    fn a() -> Self::B<'static>;
}

struct C;

struct Wrapper<T>(T);

impl A for C {
    type B<T> = Wrapper<T>;
// { dg-error "" "" { target *-*-* } .-1 }
    fn a() -> Self::B<'static> {}
}

fn main() {}

