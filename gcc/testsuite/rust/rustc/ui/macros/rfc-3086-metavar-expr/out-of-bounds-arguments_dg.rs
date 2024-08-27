#![feature(macro_metavar_expr)]

macro_rules! a {
    ( $( { $( [ $( ( $( $foo:ident )* ) )* ] )* } )* ) => {
        (
            ${count($foo, 0)},
            ${count($foo, 10)},
// { dg-error "" "" { target *-*-* } .-1 }
        )
    };
}

macro_rules! b {
    ( $( { $( [ $( $foo:ident )* ] )* } )* ) => {
        (
            $( $( $(
                ${ignore($foo)}
                ${index(0)},
                ${index(10)},
// { dg-error "" "" { target *-*-* } .-1 }
            )* )* )*
        )
    };
}

macro_rules! c {
    ( $( { $( $foo:ident )* } )* ) => {
        (
            $( $(
                ${ignore($foo)}
                ${len(0)}
                ${len(10)}
// { dg-error "" "" { target *-*-* } .-1 }
            )* )*
        )
    };
}

fn main() {
    a!( { [ (a) ] [ (b c) ] } );
    b!( { [ a b ] } );
    c!({ a });
}

