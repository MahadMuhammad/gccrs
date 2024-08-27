//@ run-pass
//@ run-rustfix
//@ rustfix-only-machine-applicable

#[allow(unused_must_use, unused_allocation)]
fn main() {
    let boxed = vec![1, 2].into_boxed_slice();

    // Expressions that should trigger the lint
    boxed.into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    Box::new(boxed.clone()).into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    Box::new(Box::new(boxed.clone())).into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // Expressions that should not
    (&boxed).into_iter();

    for _ in &boxed {}
    (&boxed as &[_]).into_iter();
    boxed[..].into_iter();
    std::iter::IntoIterator::into_iter(&boxed);

    #[allow(boxed_slice_into_iter)]
    boxed.into_iter();
}

