#![feature(inherent_associated_types, non_lifetime_binders, type_alias_impl_trait)]
#![allow(incomplete_features)]

struct Lexer<T>(T);

impl Lexer<i32> {
    type Cursor = ();
}

type X = impl for<T> Fn() -> Lexer<T>::Cursor;
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
// { dg-error ".E0220." "" { target *-*-* } .-3 }

fn main() {}

