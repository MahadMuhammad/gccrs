fn main() {
    let unused = ();

    #![allow(unused_variables)] // { dg-error "" "" { target *-*-* } }
    fn foo() {}
}

