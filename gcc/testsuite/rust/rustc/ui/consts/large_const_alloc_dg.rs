//@ only-64bit
// on 32bit and 16bit platforms it is plausible that the maximum allocation size will succeed

const FOO: () = {
    // 128 TiB, unlikely anyone has that much RAM
    let x = [0_u8; (1 << 47) - 1];
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};

static FOO2: () = {
    let x = [0_u8; (1 << 47) - 1];
// { dg-error ".E0080." "" { target *-*-* } .-1 }
};

fn main() {
    FOO;
    FOO2;
}

