type A = for<'b, 'a: 'b> fn(); // { dg-error "" "" { target *-*-* } }
type B = for<'b, 'a: 'b,> fn(); // { dg-error "" "" { target *-*-* } }
type C = for<'b, 'a: 'b +> fn(); // { dg-error "" "" { target *-*-* } }
type D = for<'a, T> fn(); // { dg-error ".E0658." "" { target *-*-* } }
type E = dyn for<T, U> Fn(); // { dg-error ".E0658." "" { target *-*-* } }

fn main() {}

