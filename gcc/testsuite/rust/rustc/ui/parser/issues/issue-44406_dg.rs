macro_rules! foo {
    ($rest: tt) => {
        bar(baz: $rest) // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {
    foo!(true);
}

