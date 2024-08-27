struct MyS;

impl MyS {
    const FOO: i32 = 1;
    fn foo() -> MyS {
        MyS
    }
}

fn main() {
    let x: i32 = MyS::foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let z: i32 = i32::max;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    // This example is still broken though... This is a hard suggestion to make,
    // because we don't have access to the associated const probing code to make
    // this suggestion where it's emitted, i.e. in trait selection.
    let y: i32 = i32::max - 42;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
// { help ".E0369." "" { target *-*-* } .-2 }
}

