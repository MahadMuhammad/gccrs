fn main() {
    f<'a,>
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn bar(a: usize, b: usize) -> usize {
    a + b
}

fn foo() {
    let x = 1;
    bar('y, x);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
}

