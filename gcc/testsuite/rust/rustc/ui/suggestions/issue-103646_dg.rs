trait Cat {
    fn nya() {}
}

fn uwu<T: Cat>(c: T) {
    c.nya();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { suggestion ".E0599." "" { target *-*-* } .-2 }
}

fn main() {}

