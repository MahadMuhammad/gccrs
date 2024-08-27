// { dg-additional-options "-frust-edition=2021" }

// Test that arrays are completely captured by closures by relying on the borrow check diagnostics

fn arrays_1() {
    let mut arr = [1, 2, 3, 4, 5];

    let mut c = || {
        arr[0] += 10;
    };

    // c will capture `arr` completely, therefore another index into the
    // array can't be modified here
    arr[1] += 10;
// { dg-error ".E0503." "" { target *-*-* } .-1 }
// { dg-error ".E0503." "" { target *-*-* } .-2 }
    c();
}

fn arrays_2() {
    let mut arr = [1, 2, 3, 4, 5];

    let c = || {
        println!("{:#?}", &arr[3..4]);
    };

    // c will capture `arr` completely, therefore another index into the
    // array can't be modified here
    arr[1] += 10;
// { dg-error ".E0506." "" { target *-*-* } .-1 }
    c();
}

fn arrays_3() {
    let mut arr = [1, 2, 3, 4, 5];

    let c = || {
        println!("{}", arr[3]);
    };

    // c will capture `arr` completely, therefore another index into the
    // array can't be modified here
    arr[1] += 10;
// { dg-error ".E0506." "" { target *-*-* } .-1 }
    c();
}

fn arrays_4() {
    let mut arr = [1, 2, 3, 4, 5];

    let mut c = || {
        arr[1] += 10;
    };

    // c will capture `arr` completely, therefore we cannot borrow another index
    // into the array.
    println!("{}", arr[3]);
// { dg-error ".E0502." "" { target *-*-* } .-1 }
// { dg-error ".E0502." "" { target *-*-* } .-2 }

    c();
}

fn arrays_5() {
    let mut arr = [1, 2, 3, 4, 5];

    let mut c = || {
        arr[1] += 10;
    };

    // c will capture `arr` completely, therefore we cannot borrow other indices
    // into the array.
    println!("{:#?}", &arr[3..2]);
// { dg-error ".E0502." "" { target *-*-* } .-1 }

    c();
}

fn main() {
    arrays_1();
    arrays_2();
    arrays_3();
    arrays_4();
    arrays_5();
}

