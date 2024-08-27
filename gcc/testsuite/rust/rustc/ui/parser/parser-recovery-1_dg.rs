// Test that we can recover from missing braces in the parser.

trait Foo {
    fn bar() {
        let x = foo();
}

fn main() {
    let x = y.;
} // { dg-error "" "" { target *-*-* } }

