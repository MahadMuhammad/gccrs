pub mod a {
    mod b {
        pub trait Hidden {}
    }
}

struct S;
impl a::b::Hidden for S {} // { dg-error ".E0603." "" { target *-*-* } }

fn main() {}

