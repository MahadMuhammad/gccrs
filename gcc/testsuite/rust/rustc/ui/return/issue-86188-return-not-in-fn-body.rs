// Due to a compiler bug, if a return occurs outside of a function body
// (e.g. in an AnonConst body), the return value expression would not be
// type-checked, leading to an ICE. This test checks that the ICE no
// longer happens, and that an appropriate error message is issued that
// also explains why the return is considered "outside of a function body"
// if it seems to be inside one, as in the main function below.

const C: [(); 42] = {
    [(); return || {
// { dg-error ".E0572." "" { target *-*-* } .-1 }
        let tx;
    }]
};

struct S {}
trait Tr {
    fn foo();
    fn bar() {
// { dg-note "" "" { target *-*-* } .-1 }
        [(); return];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }
    }
}
impl Tr for S {
    fn foo() {
// { dg-note "" "" { target *-*-* } .-1 }
        [(); return];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }
    }
}

fn main() {
// { dg-note "" "" { target *-*-* } .-1 }
    [(); return || {
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }
        let tx;
    }];
}

