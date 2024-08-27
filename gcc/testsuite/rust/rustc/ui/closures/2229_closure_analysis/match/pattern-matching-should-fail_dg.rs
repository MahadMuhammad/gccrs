// { dg-additional-options "-frust-edition=2021" }

#![feature(never_type)]

// Should fake read the discriminant and throw an error
fn test1() {
    let x: !;
    let c1 = || match x { };
// { dg-error ".E0381." "" { target *-*-* } .-1 }
}

// Should fake read the discriminant and throw an error
fn test2() {
    let x: !;
    let c2 = || match x { _ => () };
// { dg-error ".E0381." "" { target *-*-* } .-1 }
}

// Testing single variant patterns
enum SingleVariant {
    Points(u32)
}

// Should fake read the discriminant and throw an error
fn test3() {
    let variant: !;
    let c = || {
// { dg-error ".E0381." "" { target *-*-* } .-1 }
        match variant {
            SingleVariant::Points(_) => {}
        }
    };
    c();
}

// Should fake read the discriminant and throw an error
fn test4() {
    let variant: !;
    let c = || { // { dg-error ".E0381." "" { target *-*-* } }
        match variant {
            SingleVariant::Points(a) => {
                println!("{:?}", a);
            }
        }
    };
    c();
}

fn test5() {
    let t: !;
    let g: !;

    let a = || {
        match g { }; // { dg-error ".E0381." "" { target *-*-* } }
        let c = ||  {
            match t { }; // { dg-error ".E0381." "" { target *-*-* } }
        };

        c();
    };

}

// Should fake read the discriminant and throw an error
fn test6() {
    let x: u8;
    let c1 = || match x { };
// { dg-error ".E0381." "" { target *-*-* } .-1 }
// { dg-error ".E0381." "" { target *-*-* } .-2 }
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

