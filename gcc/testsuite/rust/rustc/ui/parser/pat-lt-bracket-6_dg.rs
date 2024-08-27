fn main() {
    struct Test(&'static u8, [u8; 0]);
    let x = Test(&0, []);

    let Test(&desc[..]) = x;
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { dg-error ".E0023." "" { target *-*-* } .-2 }
}

const RECOVERY_WITNESS: () = 0; // { dg-error ".E0308." "" { target *-*-* } }

