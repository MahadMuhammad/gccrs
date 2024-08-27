//@ run-rustfix
fn main() {
    let foo = 42u32;
    #[allow(unused_variables, non_snake_case)]
    const FOO : u32 = foo;
// { dg-error ".E0435." "" { target *-*-* } .-1 }
}

