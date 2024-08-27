fn main() {
    let mut x = 8u8;
    let z: *const u8 = &x;
    // issue #21596
    println!("{}", z.to_string()); // { dg-error ".E0599." "" { target *-*-* } }

    let t: *mut u8 = &mut x;
    println!("{}", t.to_string()); // { dg-error ".E0599." "" { target *-*-* } }
    t.make_ascii_lowercase(); // { dg-error ".E0599." "" { target *-*-* } }

    // suggest `as_mut` simply because the name is similar
    let _ = t.as_mut_ref(); // { dg-error ".E0599." "" { target *-*-* } }
    let _ = t.as_ref_mut(); // { dg-error ".E0599." "" { target *-*-* } }

    // no ptr-to-ref suggestion
    z.make_ascii_lowercase(); // { dg-error ".E0599." "" { target *-*-* } }
}

