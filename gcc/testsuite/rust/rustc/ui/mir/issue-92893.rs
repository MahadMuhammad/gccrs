struct Bug<A = [(); (let a = (), 1).1]> {
// { dg-error "" "" { target *-*-* } .-1 }
    a: A
}

fn main() {}

