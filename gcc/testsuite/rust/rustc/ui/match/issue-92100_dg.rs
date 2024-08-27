#![feature(half_open_range_patterns_in_slices)]

fn main() {
    match [1, 2] {
        [a.., a] => {} // { dg-error ".E0425." "" { target *-*-* } }
    }
}

