macro_rules! borrow {
    ($x:expr) => { &$x }
}

fn foo(_: String) {}

fn foo2(s: &String) {
    foo(s);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn foo3(_: u32) {}
fn foo4(u: &u32) {
    foo3(u);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

struct S<'a> {
    u: &'a u32,
}

struct R {
    i: u32,
}

fn main() {
    let s = String::new();
    let r_s = &s;
    foo2(r_s);
    foo(&"aaa".to_owned());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    foo(&mut "aaa".to_owned());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    foo3(borrow!(0));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    foo4(&0);
    assert_eq!(3i32, &3i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let u = 3;
    let s = S { u };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let s = S { u: u };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let i = &4;
    let r = R { i };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let r = R { i: i };
// { dg-error ".E0308." "" { target *-*-* } .-1 }


    let a = &1;
    let b = &2;
    let val: i32 = if true {
        a + 1
    } else {
        b
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
    let val: i32 = if true {
        let _ = 2;
        a + 1
    } else {
        let _ = 2;
        b
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
    let val = if true {
        *a
    } else if true {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        b
    } else {
        &0
    };

    #[derive(PartialEq, Eq)]
    struct Foo;
    let foo = Foo;
    let bar = &Foo;

    if foo == bar {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

