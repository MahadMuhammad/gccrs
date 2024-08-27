// { dg-additional-options "-frust-edition=2018" }

#![feature(ptr_metadata)]
#![feature(type_alias_impl_trait)]

type Opaque = impl std::fmt::Debug + ?Sized;

fn opaque() -> &'static Opaque {
    &[1] as &[i32]
}

fn a<T: ?Sized>() {
    is_thin::<T>();
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    is_thin::<Opaque>();
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

fn is_thin<T: std::ptr::Pointee<Metadata = ()> + ?Sized>() {}

fn main() {}

