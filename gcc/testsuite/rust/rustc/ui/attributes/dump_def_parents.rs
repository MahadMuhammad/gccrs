//@ normalize-stderr-test: "DefId\(.+?\)" -> "DefId(..)"
#![feature(rustc_attrs)]

fn bar() {
    fn foo() {
        fn baz() {
            #[rustc_dump_def_parents]
            || {
// { dg-error "" "" { target *-*-* } .-1 }
                qux::<
                    {
// { dg-error "" "" { target *-*-* } .-1 }
                        fn inhibits_dump() {
                            qux::<
                                {
                                    "hi";
                                    1
                                },
                            >();
                        }

                        qux::<{ 1 + 1 }>();
// { dg-error "" "" { target *-*-* } .-1 }
                        1
                    },
                >();
            };
        }
    }
}

const fn qux<const N: usize>() {}

fn main() {}

