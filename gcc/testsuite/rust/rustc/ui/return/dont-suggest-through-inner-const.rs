const fn f() -> usize {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    const FIELD: usize = loop {
        0
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

fn main() {}

