#![feature(trait_upcasting)]

trait Foo<'a>: Bar<'a> {}
trait Bar<'a> {}

fn test_correct(x: &dyn Foo<'static>) {
    let _ = x as &dyn Bar<'static>;
}

fn test_wrong1<'a>(x: &dyn Foo<'static>, y: &'a u32) {
    let _ = x as &dyn Bar<'a>; // Error
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_wrong2<'a>(x: &dyn Foo<'a>) {
    let _ = x as &dyn Bar<'static>; // Error
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

