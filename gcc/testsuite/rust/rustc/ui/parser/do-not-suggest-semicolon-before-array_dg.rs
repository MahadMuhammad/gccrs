fn foo() {}

fn bar() -> [u8; 2] {
    foo()
    [1, 3) // { dg-error "" "" { target *-*-* } }
}

fn main() {}

