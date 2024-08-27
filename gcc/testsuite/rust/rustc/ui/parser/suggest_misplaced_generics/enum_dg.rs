// Issue: 103366 , Suggest fix for misplaced generic params
//@ run-rustfix

#[allow(unused)]
enum<T> Foo { Variant(T) }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

fn main() {}

