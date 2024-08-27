fn main() {
    extern crate test; // { dg-error ".E0658." "" { target *-*-* } }
    use test::*; // { dg-error ".E0432." "" { target *-*-* } }
}

