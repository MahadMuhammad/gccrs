use std::time::Instant;

enum Enum {
    First
}

#[derive(Clone)]
enum CloneEnum {
    First
}

struct Struct {
}

#[derive(Clone)]
struct CloneStruct {
}

struct Foo<X, Y> (X, Y);
impl<X: Clone + Default + , Y: Clone + Default> Foo<X, Y> {
    fn test(&self) -> (X, Y) {
        (self.0.clone(), self.1.clone())
    }
}

fn test1() {
    let x = Foo(Enum::First, CloneEnum::First);
    let y = x.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn test2() {
    let x = Foo(Struct{}, CloneStruct{});
    let y = x.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn test3() {
    let x = Foo(Vec::<Enum>::new(), Instant::now());
    let y = x.test();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

