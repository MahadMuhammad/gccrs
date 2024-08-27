// Regression test for #87456.

fn take_mut(_val: impl FnMut()) {}

fn main() {
    let val = String::new();
// { dg-note "" "" { target *-*-* } .-1 }
    take_mut(|| {
// { dg-note "" "" { target *-*-* } .-1 }
        let _foo: String = val;
// { dg-error ".E0507." "" { target *-*-* } .-1 }
// { dg-note ".E0507." "" { target *-*-* } .-2 }
    })
}

