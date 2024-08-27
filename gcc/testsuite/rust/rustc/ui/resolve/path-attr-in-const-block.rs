// issue#126516
// issue#126647

fn main() {
    const {
        #![path = foo!()]
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

