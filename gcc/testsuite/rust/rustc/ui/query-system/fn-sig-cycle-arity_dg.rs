trait Dancer {
    fn dance(&self) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        self.dance()
    }
}

fn main() {}

