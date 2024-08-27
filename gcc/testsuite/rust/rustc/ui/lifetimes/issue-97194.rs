extern "C" {
    fn bget(&self, index: [usize; Self::DIM]) -> bool {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }
// { dg-error ".E0433." "" { target *-*-* } .-3 }
        type T<'a> = &'a str;
    }
}

fn main() {}

