fn main() {
    let x = Some(123);
    if let Some(y) = x else { // { dg-error "" "" { target *-*-* } }
        return;
    };
}

