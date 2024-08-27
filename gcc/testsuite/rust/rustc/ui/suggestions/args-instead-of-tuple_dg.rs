// Test suggesting tuples where bare arguments may have been passed
// See issue #86481 for details.

//@ run-rustfix

fn main() {
    let _: Result<(i32, i8), ()> = Ok(1, 2);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    let _: Option<(i32, i8, &'static str)> = Some(1, 2, "hi");
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    let _: Option<()> = Some();
// { dg-error ".E0061." "" { target *-*-* } .-1 }

    let _: Option<(i32,)> = Some(3);
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let _: Option<(i32,)> = Some((3));
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    two_ints(1, 2); // { dg-error ".E0061." "" { target *-*-* } }

    with_generic(3, 4); // { dg-error ".E0061." "" { target *-*-* } }
}

fn two_ints(_: (i32, i32)) {
}

fn with_generic<T: Copy + Send>((a, b): (i32, T)) {
    if false {
        // test generics/bound handling
        with_generic(a, b); // { dg-error ".E0061." "" { target *-*-* } }
    }
}

