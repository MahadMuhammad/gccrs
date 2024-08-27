macro_rules! cbor_map {
    ($key:expr) => {
        $key.signum();
// { dg-error ".E0689." "" { target *-*-* } .-1 }
    };
}

fn main() {
    cbor_map! { #[cfg(test)] 4};
// { dg-error "" "" { target *-*-* } .-1 }
}

