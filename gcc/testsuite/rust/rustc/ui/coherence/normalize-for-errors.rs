//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

struct MyType;
trait MyTrait<S> {}

trait Mirror {
    type Assoc;
}
impl<T> Mirror for T {
    type Assoc = T;
}

impl<T: Copy, S: Iterator> MyTrait<S> for (T, S::Item) {}
// { dg-note "" "" { target *-*-* } .-1 }
impl<S: Iterator> MyTrait<S> for (Box<<(MyType,) as Mirror>::Assoc>, S::Item) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

fn main() {}

