// { dg-additional-options "-frust-edition=2021" }

fn main() {
    panic!(123); // { dg-error "" "" { target *-*-* } }
    panic!("{}"); // { dg-error "" "" { target *-*-* } }
    core::panic!("{}"); // { dg-error "" "" { target *-*-* } }
    assert!(false, 123); // { dg-error "" "" { target *-*-* } }
    assert!(false, "{}"); // { dg-error "" "" { target *-*-* } }
}

