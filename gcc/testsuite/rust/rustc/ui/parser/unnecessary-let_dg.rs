fn main() {
    for let x of [1, 2, 3] {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    match 1 {
        let 1 => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

