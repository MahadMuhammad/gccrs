// Tests that a suggestion is issued for type mismatch errors when a
// u8 is expected and a char literal which is ASCII is supplied.

fn foo(_t: u8) {}

fn main() {
    let _x: u8 = 'X';
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    foo('#');
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let _a: u8 = '\x20';
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    // Do not issue the suggestion if the char literal is a Unicode escape
    foo('\u{0080}');
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    // Do not issue the suggestion if the char literal isn't ASCII
    let _t: u8 = '€';
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    // Do not issue the suggestion if the char literal isn't ASCII
    foo('\u{1f980}');
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

