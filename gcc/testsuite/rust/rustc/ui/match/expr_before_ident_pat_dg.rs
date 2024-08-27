macro_rules! funny {
    ($a:expr, $b:ident) => {
        match [1, 2] {
            [$a, $b] => {}
        }
    };
}

fn main() {
    funny!(a, a);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

