fn main() {
    let _: unsafe fn() = || { ::std::pin::Pin::new_unchecked(&0_u8); };
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    let _: unsafe fn() = || unsafe { ::std::pin::Pin::new_unchecked(&0_u8); }; // OK
}

