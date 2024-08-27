extern "C" {
    const fn foo();
// { dg-error "" "" { target *-*-* } .-1 }
    const unsafe fn bar();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

