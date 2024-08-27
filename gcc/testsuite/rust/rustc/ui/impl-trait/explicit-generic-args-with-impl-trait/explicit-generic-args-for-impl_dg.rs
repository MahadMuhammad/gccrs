fn foo<T: ?Sized>(_f: impl AsRef<T>) {}

fn main() {
    foo::<str, String>("".to_string()); // { dg-error ".E0107." "" { target *-*-* } }
}

