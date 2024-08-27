//@ compile-flags: --edition=2021
//@ run-rustfix

pub struct Struct<T> {
    pub p: T,
}

impl<T> Struct<T> {
    pub fn method(&self) {}

    pub fn some_mutable_method(&mut self) {}
}

fn main() {
    let other_item = std::cell::RefCell::new(Struct { p: 42_u32 });

    other_item.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    other_item.some_mutable_method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    let another_item = std::sync::Mutex::new(Struct { p: 42_u32 });

    another_item.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    let another_item = std::sync::RwLock::new(Struct { p: 42_u32 });

    another_item.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    another_item.some_mutable_method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

