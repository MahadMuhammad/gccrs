// Note: it's ok to interpret 'a as 'a', but not ok to interpret 'abc as
// 'abc' because 'abc' is not a valid char literal.

fn main() {
    let c = 'a;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    let c = 'abc;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn f() {
    match 'a' {
        'a'..='b => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
        'c'..='def => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn g() {
   match 'g' {
       'g => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
       'hij => {}
// { dg-error "" "" { target *-*-* } .-1 }
       _ => {}
   }
}

fn h() {
   let x = ['a, 'b, 'cde];
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-6 }
}

