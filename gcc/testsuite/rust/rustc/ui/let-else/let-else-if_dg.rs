fn main() {
    let Some(_) = Some(()) else if true {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    } else {
        return;
    };
}

