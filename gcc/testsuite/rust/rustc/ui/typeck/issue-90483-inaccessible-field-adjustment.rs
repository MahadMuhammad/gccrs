// { dg-additional-options "-frust-edition=2021" }

mod m {
  pub struct S { foo: i32 }
  impl S {
    pub fn foo(&self) -> i32 { 42 }
  }
}

fn bar(s: &m::S) {
  || s.foo() + s.foo; // { dg-error ".E0616." "" { target *-*-* } }
}

fn main() {}

