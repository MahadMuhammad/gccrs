fn hello<Q>() -> Vec<Q> {
    Vec::<Q>::mew()
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

fn main() {}

