//@ run-rustfix
fn main() {
    let _ = vec![1, 2, 3].into_iter().map({|x|
        let y = x; // { dg-error "" "" { target *-*-* } }
        y
    });
    let _: () = foo; // { dg-error ".E0308." "" { target *-*-* } }
}
fn foo() {}

