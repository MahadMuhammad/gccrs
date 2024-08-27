//@ run-rustfix

struct Something {
    pub field: u32,
}

fn main() {
    let mut something = Something { field: 1337 };
    let _ = something.field;

    let _pointer_to_something = something as *const Something;
// { dg-error ".E0605." "" { target *-*-* } .-1 }

    let _mut_pointer_to_something = something as *mut Something;
// { dg-error ".E0605." "" { target *-*-* } .-1 }
}

