fn main() {
    match 5 {
        6 ..= 1 => { }
// { dg-error ".E0030." "" { target *-*-* } .-1 }
        _ => { }
    };

    match 5 {
        0 .. 0 => { }
// { dg-error ".E0579." "" { target *-*-* } .-1 }
        _ => { }
    };

    match 5u64 {
        0xFFFF_FFFF_FFFF_FFFF ..= 1 => { }
// { dg-error ".E0030." "" { target *-*-* } .-1 }
        _ => { }
    };
}

