struct Bug {
    V1: [(); {
        let f: impl core::future::Future<Output = u8> = async { 1 };
// { dg-error ".E0562." "" { target *-*-* } .-1 }
// { dg-error ".E0562." "" { target *-*-* } .-2 }
        1
    }],
}

fn main() {}

