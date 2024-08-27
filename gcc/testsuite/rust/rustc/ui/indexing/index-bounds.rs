//@ build-fail

fn main() {
    let _n = [64][200];
// { dg-error "" "" { target *-*-* } .-1 }

    // issue #121126, test index value between 0xFFFF_FF00 and u32::MAX
    let _n = [64][u32::MAX as usize - 1];
// { dg-error "" "" { target *-*-* } .-1 }
}

