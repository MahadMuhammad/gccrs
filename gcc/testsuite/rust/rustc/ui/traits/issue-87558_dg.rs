struct ErrorKind;
struct Error(ErrorKind);
impl Fn(&isize) for Error {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error ".E0046." "" { target *-*-* } .-3 }
// { dg-error ".E0046." "" { target *-*-* } .-4 }
    fn from() {} // { dg-error ".E0407." "" { target *-*-* } }
}

fn main() {}

