struct ErrorKind;
struct Error(ErrorKind);
impl Fn(&isize) for Error {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
// { dg-error ".E0046." "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
    fn foo<const N: usize>(&self) -> Self::B<{ N }>;
// { dg-error ".E0220." "" { target *-*-* } .-1 }
// { dg-error ".E0220." "" { target *-*-* } .-2 }
// { dg-error ".E0220." "" { target *-*-* } .-3 }
}
fn main() {}

