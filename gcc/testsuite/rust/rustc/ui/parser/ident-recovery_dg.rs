fn ,comma() {
// { dg-error "" "" { target *-*-* } .-1 }
    struct Foo {
        x: i32,,
// { dg-error "" "" { target *-*-* } .-1 }
        y: u32,
    }
}

fn break() {
// { dg-error "" "" { target *-*-* } .-1 }
    let continue = 5;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

