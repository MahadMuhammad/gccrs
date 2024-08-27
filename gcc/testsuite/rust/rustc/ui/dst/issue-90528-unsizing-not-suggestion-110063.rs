trait Test {}
impl Test for &[u8] {}

fn needs_test<T: Test>() -> T {
    panic!()
}

fn main() {
    needs_test::<[u8; 1]>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let x: [u8; 1] = needs_test();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

