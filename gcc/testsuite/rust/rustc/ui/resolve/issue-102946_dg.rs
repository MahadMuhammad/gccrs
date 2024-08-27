impl Error for str::Utf8Error {
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { dg-error ".E0223." "" { target *-*-* } .-2 }
    fn description(&self)  {}
}

fn main() {}

