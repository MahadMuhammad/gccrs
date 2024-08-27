enum Hey<A, B> {
    A(A),
    B(B),
}

struct Foo {
    bar: Option<i32>,
}

fn f() {}

fn a() -> Option<()> {
    while false {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        f();
    }
// { help "" "" { target *-*-* } .-1 }
}

fn b() -> Result<(), ()> {
    f()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

fn c() -> Option<()> {
    for _ in [1, 2] {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        f();
    }
// { help "" "" { target *-*-* } .-1 }
}

fn d() -> Option<()> {
    c()?
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
}

fn main() {
    let _: Option<()> = while false {};
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let _: Option<()> = {
        while false {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    };
    let _: Result<i32, i32> = 1;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let _: Option<i32> = 1;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let _: Hey<i32, i32> = 1;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let _: Hey<i32, bool> = false;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let bar = 1i32;
    let _ = Foo { bar };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

enum A {
    B { b: B },
}

struct A2(B);

enum B {
    Fst,
    Snd,
}

fn foo() {
    let a: A = B::Fst;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

fn bar() {
    let a: A2 = B::Fst;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

