macro_rules! cbor_map {
    ($key:expr) => {
        $key.signum();
    };
}

fn main() {
    cbor_map! { #[test(test)] 4};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
}

