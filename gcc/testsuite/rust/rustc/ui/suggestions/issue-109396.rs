fn main() {
    {
        let mut mutex = std::mem::zeroed(
// { dg-error ".E0061." "" { target *-*-* } .-1 }
            file.as_raw_fd(),
// { dg-error ".E0423." "" { target *-*-* } .-1 }
            0,
            0,
            0,
        );
    }
}

