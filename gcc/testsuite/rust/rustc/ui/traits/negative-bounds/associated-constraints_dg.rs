#![feature(negative_bounds)]

trait Trait {
    type Assoc;
}

fn test<T: !Trait<Assoc = i32>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn test2<T>() where T: !Trait<Assoc = i32> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn test3<T: !Trait<Assoc: Send>>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn test4<T>() where T: !Trait<Assoc: Send> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn test5<T>() where T: !Fn() -> i32 {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

