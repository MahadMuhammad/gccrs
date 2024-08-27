// This used to ICE in exhaustiveness checking. Explanation here:
// https://github.com/rust-lang/rust/issues/82772#issuecomment-905946768
fn main() {
    let Box { 1: _, .. }: Box<()>; // { dg-error ".E0451." "" { target *-*-* } }
    let Box { .. }: Box<()>;
}

