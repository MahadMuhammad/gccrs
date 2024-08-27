fn bug() -> impl for <'r> Fn() -> &'r () { || { &() } }
// { dg-error ".E0582." "" { target *-*-* } .-1 }
// { dg-error ".E0582." "" { target *-*-* } .-2 }

fn main() {
    let f = bug();
}

