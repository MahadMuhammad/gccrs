macro_rules! values {
    ($($token:ident($value:literal) $(as $inner:ty)? => $attr:meta,)*) => {
        #[derive(Debug)]
        pub enum TokenKind {
            $(
                #[$attr]
                $token $($inner)? = $value,
            )*
        }
    };
}
// { dg-error "" "" { target *-*-* } .-5 }
// { dg-error "" "" { target *-*-* } .-2 }

values!(STRING(1) as (String) => cfg(test),);
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

