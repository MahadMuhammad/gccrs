// Issue: 103366 , Suggest fix for misplaced generic params
//@ run-rustfix

#[allow(unused)]
trait<T> Foo {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}


fn main() {}

