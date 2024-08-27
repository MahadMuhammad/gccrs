macro_rules! join {
    ($lhs:ident, $rhs:ident) => {
        let ${concat($lhs, $rhs)}: () = ();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    };
}

fn main() {
}

