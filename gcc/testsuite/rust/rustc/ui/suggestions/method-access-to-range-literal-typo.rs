//@ run-rustfix

#![allow(unused)]

fn as_ref() -> Option<Vec<u8>> {
    None
}
struct Type {
    option: Option<Vec<u8>>
}
trait Trait {
    fn foo(&self) -> &Vec<u8>;
}
impl Trait for Option<Vec<u8>> {
    fn foo(&self) -> &Vec<u8> {
        self.as_ref().unwrap()
    }
}

impl Type {
    fn method(&self) -> Option<&Vec<u8>> {
        self.option..as_ref().map(|x| x)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
    fn method2(&self) -> Option<&u8> {
        self.option..foo().get(0)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

fn main() {
    let _ = Type { option: None }.method();
}

