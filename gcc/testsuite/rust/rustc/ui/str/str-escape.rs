//@ check-pass
// ignore-tidy-tab
// { dg-additional-options "-frust-edition= 2021" }

fn main() {
    let s = "\

             ";
// { dg-warning "" "" { target *-*-* } .-3 }
    assert_eq!(s, "");

    let s = c"foo\
             bar
             ";
// { dg-warning "" "" { target *-*-* } .-3 }
    assert_eq!(s, c"foo           bar\n             ");

    let s = "a\
 b";
    assert_eq!(s, "ab");

    let s = "a\
	b";
    assert_eq!(s, "ab");

    let s = b"a\
    b";
// { dg-warning "" "" { target *-*-* } .-2 }
    // '\x0c' is ASCII whitespace, but it may not need skipped
    // discussion: https://github.com/rust-lang/rust/pull/108403
    assert_eq!(s, b"a\x0cb");
}

