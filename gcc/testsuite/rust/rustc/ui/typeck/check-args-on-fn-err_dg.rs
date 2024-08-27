fn main() {
    unknown(1, |glyf| {
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        let actual = glyf;
    });
}

