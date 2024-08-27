fn main() {
    let x = |x|
        let y = x; // { dg-error "" "" { target *-*-* } }
        let _ = () + ();
        y
}

