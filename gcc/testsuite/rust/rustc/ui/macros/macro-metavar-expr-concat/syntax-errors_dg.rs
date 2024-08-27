#![feature(macro_metavar_expr_concat)]

macro_rules! wrong_concat_declarations {
    ($ex:expr) => {
        ${concat()}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat(aaaa)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat(aaaa,)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat(_, aaaa)}

        ${concat(aaaa aaaa)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat($ex)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat($ex, aaaa)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat($ex, aaaa 123)}
// { dg-error "" "" { target *-*-* } .-1 }

        ${concat($ex, aaaa,)}
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

macro_rules! dollar_sign_without_referenced_ident {
    ($ident:ident) => {
        const ${concat(FOO, $foo)}: i32 = 2;
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

macro_rules! starting_number {
    ($ident:ident) => {{
        let ${concat("1", $ident)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
    }};
}

macro_rules! starting_valid_unicode {
    ($ident:ident) => {{
        let ${concat("Ý", $ident)}: () = ();
    }};
}

macro_rules! starting_invalid_unicode {
    ($ident:ident) => {{
        let ${concat("\u{00BD}", $ident)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
    }};
}

macro_rules! ending_number {
    ($ident:ident) => {{
        let ${concat($ident, "1")}: () = ();
    }};
}

macro_rules! ending_valid_unicode {
    ($ident:ident) => {{
        let ${concat($ident, "Ý")}: () = ();
    }};
}

macro_rules! ending_invalid_unicode {
    ($ident:ident) => {{
        let ${concat($ident, "\u{00BD}")}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
    }};
}

macro_rules! empty {
    () => {{
        let ${concat("", "")}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
    }};
}

macro_rules! unsupported_literals {
    ($ident:ident) => {{
        let ${concat(_a, 'b')}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let ${concat(_a, 1)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }

        let ${concat($ident, 'b')}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
        let ${concat($ident, 1)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
    }};
}

macro_rules! bad_literal_string {
    ($literal:literal) => {
        const ${concat(_foo, $literal)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-6 }
// { dg-error "" "" { target *-*-* } .-7 }
    }
}

macro_rules! bad_literal_non_string {
    ($literal:literal) => {
        const ${concat(_foo, $literal)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
    }
}

macro_rules! bad_tt_literal {
    ($tt:tt) => {
        const ${concat(_foo, $tt)}: () = ();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
    }
}

fn main() {
    wrong_concat_declarations!(1);

    dollar_sign_without_referenced_ident!(VAR);

    starting_number!(_abc);
    starting_valid_unicode!(_abc);
    starting_invalid_unicode!(_abc);

    ending_number!(_abc);
    ending_valid_unicode!(_abc);
    ending_invalid_unicode!(_abc);
    unsupported_literals!(_abc);

    empty!();

    bad_literal_string!("\u{00BD}");
    bad_literal_string!("\x41");
    bad_literal_string!("🤷");
    bad_literal_string!("d[-_-]b");

    bad_literal_string!("-1");
    bad_literal_string!("1.0");
    bad_literal_string!("'1'");

    bad_literal_non_string!(1);
    bad_literal_non_string!(-1);
    bad_literal_non_string!(1.0);
    bad_literal_non_string!('1');
    bad_literal_non_string!(false);

    bad_tt_literal!(1);
    bad_tt_literal!(1.0);
    bad_tt_literal!('1');
}

