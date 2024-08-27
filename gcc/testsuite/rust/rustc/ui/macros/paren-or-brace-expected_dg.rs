macro_rules! foo {
    ( $( $i:ident ),* ) => {
        $[count($i)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    };
}

fn main() {}

