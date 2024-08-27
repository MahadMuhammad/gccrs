//@ build-fail

fn assert_zst<T>() {
    struct F<T>(T);
    impl<T> F<T> {
        const V: () = assert!(std::mem::size_of::<T>() == 0);
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-note ".E0080." "" { target *-*-* } .-2 }
// { dg-note ".E0080." "" { target *-*-* } .-3 }
// { dg-error ".E0080." "" { target *-*-* } .-4 }
// { dg-note ".E0080." "" { target *-*-* } .-5 }
// { dg-note ".E0080." "" { target *-*-* } .-6 }
    }
    F::<T>::V;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
}

fn foo<U>() {
    assert_zst::<U>()
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}


fn bar<V>() {
    foo::<V>()
}

fn main() {
    bar::<()>();
    bar::<u32>();
    bar::<u32>();
    bar::<i32>();
}

