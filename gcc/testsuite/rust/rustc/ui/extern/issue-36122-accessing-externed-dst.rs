fn main() {
    extern "C" {
        static symbol: [usize]; // { dg-error ".E0277." "" { target *-*-* } }
    }
    println!("{}", symbol[0]);
// { dg-error ".E0133." "" { target *-*-* } .-1 }
}

