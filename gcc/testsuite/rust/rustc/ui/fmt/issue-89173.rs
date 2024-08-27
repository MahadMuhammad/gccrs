// Regression test for #89173: Make sure a helpful note is issued for
// printf-style format strings using `*` to specify the width.

fn main() {
    let num = 0x0abcde;
    let width = 6;
    print!("%0*x", width, num);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
}

