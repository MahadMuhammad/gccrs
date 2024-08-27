#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }

const fn test() -> impl ~const Fn() {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    const move || { // { dg-error ".E0658." "" { target *-*-* } }
        let sl: &[u8] = b"foo";

        match sl {
            [first, remainder @ ..] => {
                assert_eq!(first, &b'f');
// { dg-error ".E0015." "" { target *-*-* } .-1 }
            }
            [] => panic!(),
        }
    }
}

fn main() {}

