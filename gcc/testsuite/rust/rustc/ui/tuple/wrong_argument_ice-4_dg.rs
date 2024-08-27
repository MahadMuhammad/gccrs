fn main() {
    (|| {})(|| {
// { dg-error ".E0057." "" { target *-*-* } .-1 }
        let b = 1;
    });
}

