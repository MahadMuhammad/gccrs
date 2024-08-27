// Another example from issue #84660, this time weaponized as a safe transmute: an opaque type in an
// impl header being accepted was used to create unsoundness.

//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

#![feature(type_alias_impl_trait)]

trait Foo {}
impl Foo for () {}
type Bar = impl Foo;
fn _defining_use() -> Bar {}

trait Trait<T, In> {
    type Out;
    fn convert(i: In) -> Self::Out;
}

impl<In, Out> Trait<Bar, In> for Out {
    type Out = Out;
    fn convert(_i: In) -> Self::Out {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        unreachable!();
    }
}

impl<In, Out> Trait<(), In> for Out {
// { dg-error "" "" { target *-*-* } .-1 }
    type Out = In;
    fn convert(i: In) -> Self::Out {
        i
    }
}

fn transmute<In, Out>(i: In) -> Out {
    <Out as Trait<Bar, In>>::convert(i)
}

fn main() {
    let d;
    {
        let x = "Hello World".to_string();
        d = transmute::<&String, &String>(&x);
    }
    println!("{}", d);
}

