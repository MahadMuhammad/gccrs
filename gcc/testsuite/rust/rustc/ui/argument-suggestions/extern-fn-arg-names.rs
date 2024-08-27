extern "Rust" {
    fn dstfn(src: i32, dst: err);
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

fn main() {
    dstfn(1);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

