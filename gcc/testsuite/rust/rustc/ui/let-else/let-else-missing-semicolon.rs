fn main() {
    let Some(x) = Some(1) else {
        return;
    } // { dg-error "" "" { target *-*-* } }
    let _ = "";
    let Some(x) = Some(1) else {
        panic!();
    } // { dg-error "" "" { target *-*-* } }
}

