// { dg-additional-options "-frust-edition= 2018" }

// tests that the pointee type of a raw pointer must be known to call methods on it
// see also: `tests/ui/editions/edition-raw-pointer-method-2018.rs`

fn main() {
    let val = 1_u32;
    let ptr = &val as *const u32;
    unsafe {
        let _a: i32 = (ptr as *const _).read();
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        let b = ptr as *const _;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        let _b: u8 = b.read();
        let _c = (ptr as *const u8).read(); // we know the type here
    }

    let mut val = 2_u32;
    let ptr = &mut val as *mut u32;
    unsafe {
        let _a: i32 = (ptr as *mut _).read();
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        let b = ptr as *mut _;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        b.write(10);
        (ptr as *mut i32).write(1000); // we know the type here
    }
}

