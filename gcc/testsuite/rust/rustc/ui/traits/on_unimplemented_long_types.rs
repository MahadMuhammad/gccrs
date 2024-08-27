//@ compile-flags: --diagnostic-width=60 -Z write-long-types-to-disk=yes
//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"

pub fn foo() -> impl std::fmt::Display {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
        Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
            Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                    Some(Some(Some(Some(Some(Some(Some(Some(())))))))),
                ))))))))))),
            ))))))))))),
        ))))))))))),
    )))))))))))
}

fn main() {}

