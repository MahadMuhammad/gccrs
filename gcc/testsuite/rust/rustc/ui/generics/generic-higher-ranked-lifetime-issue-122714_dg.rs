#![allow(dead_code)]

trait Trait1<T>
  where T: for<'a> Trait1<T> + 'b { } // { dg-error ".E0261." "" { target *-*-* } }

trait Trait2<T>
where
    T: B<'b> + for<'a> A<'a>, // { dg-error ".E0261." "" { target *-*-* } }
{
}

trait Trait3<T>
where
    T: B<'b> + for<'a> A<'a> + 'c {}
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }

trait Trait4<T>
where
    T: for<'a> A<'a> + 'x + for<'b> B<'b>, // { dg-error ".E0261." "" { target *-*-* } }
{
}

trait A<'a> {}
trait B<'a> {}


fn main() {}

