//@ compile-flags: -Zvalidate-mir -Zinline-mir=yes

#![feature(unboxed_closures)]

use std::sync::Arc;

pub struct WeakOnce<T>();
// { dg-error ".E0392." "" { target *-*-* } .-1 }

impl<T> WeakOnce<T> {
    extern "rust-call" fn try_get(&self) -> Option<Arc<T>> {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    pub fn get(&self) -> Arc<T> {
        self.try_get()
            .unwrap_or_else(|| panic!("Singleton {} not available", std::any::type_name::<T>()))
    }
}

fn main() {}

