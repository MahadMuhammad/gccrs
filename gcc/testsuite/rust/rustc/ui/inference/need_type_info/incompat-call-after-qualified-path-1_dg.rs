// issue#121613

#![feature(more_qualified_paths)]

struct S<T> {
    a: T
}

struct Foo;

trait A {
    type Assoc<T>;
}

impl A for Foo {
    type Assoc<T> = S<T>;
}

fn f() {}

fn main() {
  <Foo as A>::Assoc::<i32> {
    a: 1
  };
  f(|a, b| a.cmp(b));
// { dg-error ".E0061." "" { target *-*-* } .-1 }
// { dg-error ".E0061." "" { target *-*-* } .-2 }
}

