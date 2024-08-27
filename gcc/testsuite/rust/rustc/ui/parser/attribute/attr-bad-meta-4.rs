macro_rules! mac {
    ($attr_item: meta) => {
        #[cfg($attr_item)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        struct S;
    }
}

mac!(an(arbitrary token stream));

#[cfg(feature = -1)]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
fn handler() {}

fn main() {}

