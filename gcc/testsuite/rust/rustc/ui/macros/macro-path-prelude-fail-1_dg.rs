mod m {
    fn check() {
        Vec::clone!(); // { dg-error ".E0433." "" { target *-*-* } }
        u8::clone!(); // { dg-error ".E0433." "" { target *-*-* } }
    }
}

fn main() {}

