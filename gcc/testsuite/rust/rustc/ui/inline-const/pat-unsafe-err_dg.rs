#![feature(inline_const_pat)]

const unsafe fn require_unsafe() -> usize {
    1
}

fn main() {
    match () {
        const {
            require_unsafe();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
        } => (),
    }

    match 1 {
        const {
            require_unsafe()
// { dg-error ".E0133." "" { target *-*-* } .-1 }
        }..=4 => (),
        _ => (),
    }
}

