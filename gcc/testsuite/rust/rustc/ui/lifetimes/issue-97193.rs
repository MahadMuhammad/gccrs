extern "C" {
    fn a(&mut self) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        fn b(buf: &Self) {}
    }
}

fn main() {}

