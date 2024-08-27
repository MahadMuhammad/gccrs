//@ check-pass
// { dg-additional-options "-frust-edition=2018" }

use std::array::IntoIter;
use std::ops::Deref;
use std::rc::Rc;
use std::slice::Iter;

fn main() {
    let array = [0; 10];

    // Before 2021, the method dispatched to `IntoIterator for &[T; N]`,
    // which we continue to support for compatibility.
    let _: Iter<'_, i32> = array.into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    let _: Iter<'_, i32> = Box::new(array).into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    let _: Iter<'_, i32> = Rc::new(array).into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    let _: Iter<'_, i32> = Array(array).into_iter();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // But you can always use the trait method explicitly as an array.
    let _: IntoIter<i32, 10> = IntoIterator::into_iter(array);

    for _ in [1, 2, 3].into_iter() {}
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

/// User type that dereferences to an array.
struct Array([i32; 10]);

impl Deref for Array {
    type Target = [i32; 10];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

