// { dg-additional-options "-frust-edition=2021" }

macro_rules! demo2 {
    ( $a:tt $b:tt ) => { println!("two tokens") };
}

macro_rules! demo3 {
    ( $a:tt $b:tt $c:tt ) => { println!("three tokens") };
}

macro_rules! demo4 {
    ( $a:tt $b:tt $c:tt $d:tt ) => { println!("four tokens") };
}

fn main() {
    demo3!(foo#bar);   // { dg-error "" "" { target *-*-* } }
    demo2!(foo"bar");  // { dg-error "" "" { target *-*-* } }
    demo2!(foo'b');    // { dg-error "" "" { target *-*-* } }

    demo2!(foo'b);     // { dg-error "" "" { target *-*-* } }
    demo3!(foo# bar);  // { dg-error "" "" { target *-*-* } }
    demo4!(foo#! bar); // { dg-error "" "" { target *-*-* } }
    demo4!(foo## bar); // { dg-error "" "" { target *-*-* } }

    demo4!(foo#bar#);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

    demo3!(foo # bar);
    demo3!(foo #bar);
    demo4!(foo!#bar);
    demo4!(foo ##bar);

    demo3!(r"foo"#bar);
    demo3!(r#foo#bar);
}

