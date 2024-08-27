macro_rules! construct { ($x:ident) => { $x"str" } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

macro_rules! contain { () => { c"str" } }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }

fn check_macro_construct() {
    construct!(c); // { dg-note "" "" { target *-*-* } }
}

fn check_macro_contain() {
    contain!();
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
}

fn check_basic() {
    c"str";
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
}

fn check_craw() {
    cr"str";
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
}

fn check_craw_hash() {
    cr##"str"##;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
}

fn check_cstr_space() {
    c "str";
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

fn main() {}

