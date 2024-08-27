#[link(kind = "dylib")] extern "C" {} // { dg-error ".E0459." "" { target *-*-* } }

fn main() {
}

