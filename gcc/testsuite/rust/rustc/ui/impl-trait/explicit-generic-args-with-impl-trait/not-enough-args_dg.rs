fn f<T: ?Sized, U: ?Sized>(_: impl AsRef<T>, _: impl AsRef<U>) {}

fn main() {
    f::<[u8]>("a", b"a");
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

