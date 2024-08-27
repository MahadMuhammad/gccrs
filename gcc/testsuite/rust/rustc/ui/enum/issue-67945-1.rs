enum Bug<S> { // { dg-error ".E0392." "" { target *-*-* } }
    Var = {
        let x: S = 0; // { dg-error "" "" { target *-*-* } }
        0
    },
}

fn main() {}

