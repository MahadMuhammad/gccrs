use std::env;

pub struct Foo {
    text: String
}

pub fn foo() -> Foo {
    let args: Vec<String> = env::args().collect();
    let text = args[1].clone();

    pub Foo { text }
}
// { dg-error "" "" { target *-*-* } .-2 }

fn main() {}

