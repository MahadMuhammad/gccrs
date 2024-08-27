fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
// { dg-error "" "" { target *-*-* } .-1 }
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}

