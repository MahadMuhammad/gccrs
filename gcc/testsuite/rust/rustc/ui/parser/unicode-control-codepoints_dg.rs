fn main() {
    // if access_level != "us‫e‪r" { // Check if admin
// { dg-error "" "" { target *-*-* } .-1 }
    println!("us\u{202B}e\u{202A}r");
    println!("{:?}", r#"us\u{202B}e\u{202A}r"#);
    println!("{:?}", b"us\u{202B}e\u{202A}r");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    println!("{:?}", br##"us\u{202B}e\u{202A}r"##);

    println!("{:?}", "/*‮ } ⁦if isAdmin⁩ ⁦ begin admins only ");
// { dg-error "" "" { target *-*-* } .-1 }

    println!("{:?}", r##"/*‮ } ⁦if isAdmin⁩ ⁦ begin admins only "##);
// { dg-error "" "" { target *-*-* } .-1 }
    println!("{:?}", b"/*‮ } ⁦if isAdmin⁩ ⁦ begin admins only ");
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
    println!("{:?}", br##"/*‮ } ⁦if isAdmin⁩ ⁦ begin admins only "##);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
    println!("{:?}", '‮');
// { dg-error "" "" { target *-*-* } .-1 }
}

//"/*‮ } ⁦if isAdmin⁩ ⁦ begin admins only */"
// { dg-error "" "" { target *-*-* } .-1 }

/**  '‮'); */fn foo() {}
// { dg-error "" "" { target *-*-* } .-1 }

/**
 *
 *  '‮'); */fn bar() {}
// { dg-error "" "" { target *-*-* } .-3 }

