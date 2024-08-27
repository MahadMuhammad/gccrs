struct Foo {
    let x: i32,
// { dg-error "" "" { target *-*-* } .-1 }
    let y: i32,
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let _ = Foo {
// { dg-error ".E0063." "" { target *-*-* } .-1 }
    };
}

