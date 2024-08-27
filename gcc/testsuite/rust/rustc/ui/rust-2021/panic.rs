// { dg-additional-options "-frust-edition=2021" }

fn main() {
    debug_assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    panic!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }

    std::debug_assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    std::assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    std::panic!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }

    core::debug_assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    core::assert!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
    core::panic!(false, 123);
// { dg-error "" "" { target *-*-* } .-1 }
}

