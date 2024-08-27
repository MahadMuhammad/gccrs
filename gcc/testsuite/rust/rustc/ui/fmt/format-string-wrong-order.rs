fn main() {
    let bar = 3;
    format!("{?:}", bar);
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{?:bar}");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{?:?}", bar);
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{??}", bar);
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{?;bar}");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{?:#?}", bar);
// { dg-error "" "" { target *-*-* } .-1 }
    format!("Hello {<5:}!", "x");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("Hello {^5:}!", "x");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("Hello {>5:}!", "x");
// { dg-error "" "" { target *-*-* } .-1 }
}

