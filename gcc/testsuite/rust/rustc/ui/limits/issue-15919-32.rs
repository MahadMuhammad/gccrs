//@ ignore-64bit
//@ build-fail

fn main() {
    let x = [0usize; 0xffff_ffff]; // { dg-error "" "" { target *-*-* } }
}

// This and the -64 version of this test need to have different literals, as we can't rely on
// conditional compilation for them while retaining the same spans/lines.

