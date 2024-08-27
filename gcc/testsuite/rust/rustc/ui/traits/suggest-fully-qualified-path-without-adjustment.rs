use std::ops::{Deref, DerefMut};

struct Thing;

trait Method<T> {
    fn method(&self) -> T;
    fn mut_method(&mut self) -> T;
}

impl Method<i32> for Thing {
    fn method(&self) -> i32 { 0 }
    fn mut_method(&mut self) -> i32 { 0 }
}

impl Method<u32> for Thing {
    fn method(&self) -> u32 { 0 }
    fn mut_method(&mut self) -> u32 { 0 }
}

trait MethodRef<T> {
    fn by_self(self);
}
impl MethodRef<i32> for &Thing {
    fn by_self(self) {}
}
impl MethodRef<u32> for &Thing {
    fn by_self(self) {}
}

struct DerefsTo<T>(T);
impl<T> Deref for DerefsTo<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for DerefsTo<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut ref_thing = &Thing;
    ref_thing.method(); // { dg-error ".E0283." "" { target *-*-* } }
    ref_thing.by_self(); // { dg-error ".E0283." "" { target *-*-* } }

    let mut mut_thing = &mut Thing;
    mut_thing.method(); // { dg-error ".E0283." "" { target *-*-* } }
    mut_thing.mut_method(); // { dg-error ".E0283." "" { target *-*-* } }
    mut_thing.by_self(); // { dg-error ".E0283." "" { target *-*-* } }

    let mut deref_to = &DerefsTo(Thing);
    deref_to.method(); // { dg-error ".E0283." "" { target *-*-* } }
    deref_to.mut_method(); // { dg-error ".E0283." "" { target *-*-* } }
    deref_to.by_self(); // { dg-error ".E0283." "" { target *-*-* } }

    let mut deref_deref_to = &DerefsTo(DerefsTo(Thing));
    deref_deref_to.method(); // { dg-error ".E0283." "" { target *-*-* } }
    deref_deref_to.mut_method(); // { dg-error ".E0283." "" { target *-*-* } }
    deref_deref_to.by_self(); // { dg-error ".E0283." "" { target *-*-* } }
}

