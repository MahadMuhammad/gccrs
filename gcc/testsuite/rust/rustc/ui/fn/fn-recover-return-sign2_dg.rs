// Separate test file because `Fn() => bool` isn't getting fixed and rustfix complained that
// even though a fix was applied the code was still incorrect

fn foo() => impl Fn() => bool {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    unimplemented!()
}

