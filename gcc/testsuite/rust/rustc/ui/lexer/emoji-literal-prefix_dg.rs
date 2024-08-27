macro_rules! lexes {($($_:tt)*) => {}}

lexes!(🐛#); // { dg-error "" "" { target *-*-* } }
lexes!(🐛"foo");
lexes!(🐛'q');
lexes!(🐛'q);

fn main() {}

