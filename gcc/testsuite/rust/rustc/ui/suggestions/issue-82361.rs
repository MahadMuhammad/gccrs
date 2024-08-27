//@ run-rustfix

fn main() {
    let a: usize = 123;
    let b: &usize = &a;

    if true {
        a
    } else {
        b // { dg-error ".E0308." "" { target *-*-* } }
    };

    if true {
        1
    } else {
        &1 // { dg-error ".E0308." "" { target *-*-* } }
    };

    if true {
        1
    } else {
        &mut 1 // { dg-error ".E0308." "" { target *-*-* } }
    };
}

