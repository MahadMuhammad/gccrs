//@ run-rustfix
fn main () {
    #[allow(non_upper_case_globals)]
    let foo: usize = 42;
    let _: [u8; foo]; // { dg-error ".E0435." "" { target *-*-* } }
}

