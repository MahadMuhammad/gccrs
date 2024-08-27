struct X {,} // { dg-error "" "" { target *-*-* } }

fn main() {
    || {
        if let X { x: 1,} = (X {}) {}
    };
}

