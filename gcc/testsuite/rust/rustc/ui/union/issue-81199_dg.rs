#[repr(C)]
union PtrRepr<T: ?Sized> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    components: PtrComponents<T>,
// { dg-error ".E0740." "" { target *-*-* } .-1 }
// { dg-error ".E0740." "" { target *-*-* } .-2 }
}

#[repr(C)]
struct PtrComponents<T: Pointee + ?Sized> {
    data_pointer: *const (),
    metadata: <T as Pointee>::Metadata,
}



pub trait Pointee {
   type Metadata;
}

fn main() {}

