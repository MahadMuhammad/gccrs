pub mod foo {
    pub mod bar {
        pub struct A;
    }
}

pub struct Foo {
  a: Vec<foo::bar:A>,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {}

