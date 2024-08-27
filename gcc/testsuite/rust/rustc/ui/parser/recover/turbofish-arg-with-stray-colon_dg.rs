fn foo() {
    let x = Tr<A, A:>;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

