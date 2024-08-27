fn fn_once() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
// { dg-note ".E0121." "" { target *-*-* } .-5 }
    let x = String::new();
    || {
        drop(x);
    }
}

fn fn_mut() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
// { dg-note ".E0121." "" { target *-*-* } .-5 }
    let x = String::new();
// { help "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    |c| { // { dg-note "" "" { target *-*-* } }
        x.push(c);
// { dg-error ".E0597." "" { target *-*-* } .-1 }
// { dg-note ".E0597." "" { target *-*-* } .-2 }
// { dg-note ".E0597." "" { target *-*-* } .-3 }
// { dg-error ".E0597." "" { target *-*-* } .-4 }
    }
} // { dg-note "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }

fn fun() -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-note ".E0121." "" { target *-*-* } .-2 }
// { help ".E0121." "" { target *-*-* } .-3 }
// { suggestion ".E0121." "" { target *-*-* } .-4 }
// { dg-note ".E0121." "" { target *-*-* } .-5 }
    || 1i32
}

fn main() {}

