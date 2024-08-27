// Ensure we don't suggest tuple-wrapping when we'd end up with a type error

fn main() {
    // we shouldn't suggest to fix these - `2` isn't a `bool`

    let _: Option<(i32, bool)> = Some(1, 2);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
    int_bool(1, 2);
// { dg-error ".E0061." "" { target *-*-* } .-1 }

    let _: Option<(i8,)> = Some();
// { dg-error ".E0061." "" { target *-*-* } .-1 }

    let _: Option<(i32,)> = Some(5_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let _: Option<(i32,)> = Some((5_usize));
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn int_bool(_: (i32, bool)) {
}

