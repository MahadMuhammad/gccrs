#![deny(large_assignments)]
#![cfg_attr(attribute, feature(large_assignments))]
#![cfg_attr(attribute, move_size_limit = "1000")]
//@ build-fail
//@ only-64bit
//@ revisions: attribute option
//@ [option]compile-flags: -Zmove-size-limit=1000

// { dg-additional-options "-frust-edition=2018" }
//@ compile-flags: -Zmir-opt-level=0

fn main() {
    let x = async {
        let y = [0; 9999];
        dbg!(y);
        thing(&y).await;
        dbg!(y);
    };
    let z = (x, 42); // { dg-error "" "" { target *-*-* } }
    let a = z.0; // { dg-error "" "" { target *-*-* } }
    let b = z.1;
}

async fn thing(y: &[u8]) {
    dbg!(y);
}

