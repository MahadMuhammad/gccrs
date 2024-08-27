//@ run-rustfix

fn func() -> u8 {
    0
}

fn main() {
    match () {
        () => func() // { dg-error ".E0308." "" { target *-*-* } }
    }
}

