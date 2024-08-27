macro_rules! foo {
    ( $f:path ) => {{
        let _: usize = $f; // { dg-error ".E0308." "" { target *-*-* } }
    }};
}

struct Baz;

fn main() {
    foo!(Baz);
}

