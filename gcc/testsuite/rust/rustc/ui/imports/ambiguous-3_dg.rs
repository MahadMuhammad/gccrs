//@ check-pass
// https://github.com/rust-lang/rust/issues/47525

fn main() {
    use a::*;
    x();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

mod a {
    mod b {
        pub fn x() { println!(module_path!()); }
    }
    mod c {
        pub fn x() { println!(module_path!()); }
    }

    pub use self::b::*;
    pub use self::c::*;
}

