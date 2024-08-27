mod m {
    // An inferred main entry point
    // must appear at the top of the crate
    fn main() { }
} // { dg-error ".E0601." "" { target *-*-* } }

