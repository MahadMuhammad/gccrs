#![feature(return_type_notation)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait Tr {
    const CONST: usize;

    fn method() -> impl Sized;
}

fn foo<T: Tr>()
where
    T::method(..): Send,
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    <T as Tr>::method(..): Send,
// { dg-error ".E0575." "" { target *-*-* } .-1 }
// { dg-error ".E0575." "" { target *-*-* } .-2 }
{
    let _ = T::CONST::(..);
// { dg-error "" "" { target *-*-* } .-1 }
    let _: T::method(..);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

