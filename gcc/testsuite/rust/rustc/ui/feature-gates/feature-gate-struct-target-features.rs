#[target_feature(enable = "avx")] // { dg-error "" "" { target *-*-* } }
struct Avx {}

fn main() {}

