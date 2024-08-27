pub fn test() {
    foo(|_|) // { dg-error ".E0425." "" { target *-*-* } }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() { }

