fn foo() {
    let _ = 0..<10;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn bar() {
    let _ = 0..<foo;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn baz() {
    let _ = <foo>;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn qux() {
    let _ = [1, 2, 3][..<1];
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn quux() {
    let _ = [1, 2, 3][..<foo];
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn foobar() {
    let _ = [1, 2, 3][..<foo>];
// { dg-error "" "" { target *-*-* } .-1 }
}

fn ok1() {
    let _ = [1, 2, 3][..<usize>::default()];
}

fn ok2() {
    let _ = 0..<i32>::default();
}

fn main() {}

