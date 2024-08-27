#![feature(type_alias_impl_trait)]

type FooArg<'a> = &'a dyn ToString;

type FooItem = Box<dyn Fn(FooArg) -> FooRet>;

#[repr(C)]
struct Bar(u8);

impl Iterator for Bar {
    type Item = FooItem;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Box::new(quux))
    }
}

mod ret {
    pub type FooRet = impl std::fmt::Debug;
    pub fn quux(st: super::FooArg) -> FooRet {
        Some(st.to_string())
    }
}
use ret::*;
mod foo {
    pub type Foo = impl Iterator<Item = super::FooItem>;
    pub fn ham() -> Foo {
        super::Bar(1)
    }
    pub fn oof(_: Foo) -> impl std::fmt::Debug {
// { dg-error "" "" { target *-*-* } .-1 }
        let mut bar = ham();
        let func = bar.next().unwrap();
        return func(&"oof");
    }
}
use foo::*;

fn main() {
    let _ = oof(ham());
}

