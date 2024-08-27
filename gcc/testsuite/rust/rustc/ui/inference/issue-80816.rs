#![allow(unreachable_code)]

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

pub struct Guard<T> {
    _phantom: PhantomData<T>,
}
impl<T> Deref for Guard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unimplemented!()
    }
}

pub struct DirectDeref<T>(T);
impl<T> Deref for DirectDeref<Arc<T>> {
    type Target = T;
    fn deref(&self) -> &T {
        unimplemented!()
    }
}

pub trait Access<T> {
    type Guard: Deref<Target = T>;
    fn load(&self) -> Self::Guard {
        unimplemented!()
    }
}
impl<T, A: Access<T>, P: Deref<Target = A>> Access<T> for P {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    type Guard = A::Guard;
}
impl<T> Access<T> for ArcSwapAny<T> {
// { dg-note "" "" { target *-*-* } .-1 }
    type Guard = Guard<T>;
}
impl<T> Access<T> for ArcSwapAny<Arc<T>> {
    type Guard = DirectDeref<Arc<T>>;
}

pub struct ArcSwapAny<T> {
    _phantom_arc: PhantomData<T>,
}

pub fn foo() {
    let s: Arc<ArcSwapAny<Arc<usize>>> = unimplemented!();
    let guard: Guard<Arc<usize>> = s.load();
// { dg-error ".E0283." "" { target *-*-* } .-1 }
// { help ".E0283." "" { target *-*-* } .-2 }
}

fn main() {}

