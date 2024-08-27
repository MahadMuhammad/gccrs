//@ run-rustfix
// https://github.com/rust-lang/rust/issues/95616

fn buggy_const<const N: usize>(_a: &Option<[u8; N]>, _f: &str) -> &str { // { dg-error ".E0106." "" { target *-*-* } }
    return "";
}

fn main() {
    buggy_const(&Some([69,69,69,69,0]), "test");
}

