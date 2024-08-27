fn main() {
    // There shall be no suggestions here. In particular not `Ok`.
    let _ = 读文; // { dg-error ".E0425." "" { target *-*-* } }

    let f = 0f32; // Important line to make this an ICE regression test
    读文(f); // { dg-error ".E0425." "" { target *-*-* } }
}

