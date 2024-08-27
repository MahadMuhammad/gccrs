// This tests that the error generated when a character literal has multiple
// characters in it contains a note about non-printing characters.

fn main() {
    let _hair_space_around = ' x​';
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { suggestion "" "" { target *-*-* } .-4 }
}

