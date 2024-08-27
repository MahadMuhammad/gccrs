fn thing() {
    let f = |_, _| ();
    f(f); // { dg-error ".E0057." "" { target *-*-* } }
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

fn main() {}

