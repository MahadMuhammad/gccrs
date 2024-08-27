//@  compile-flags: --edition=2024 -Z unstable-options

macro_rules! m {
    ($e:expr_2021) => { // { dg-error ".E0658." "" { target *-*-* } }
        $e
    };
}

fn main() {
    m!(());
}

