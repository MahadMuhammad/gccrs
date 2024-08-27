//@ run-rustfix

fn foo() -> i32? { // { dg-error "" "" { target *-*-* } }
    let x: i32? = Some(1); // { dg-error "" "" { target *-*-* } }
    x
}

fn main() {
    let _: Option<i32> = foo();
}

