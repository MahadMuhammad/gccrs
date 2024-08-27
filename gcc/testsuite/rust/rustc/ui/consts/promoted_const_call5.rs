#![feature(rustc_attrs)]
#![feature(staged_api)]
#![stable(feature = "a", since = "1.0.0")]

#[rustc_promotable]
#[stable(feature = "a", since = "1.0.0")]
#[rustc_const_stable(feature = "a", since = "1.0.0")]
pub const fn id<T>(x: &'static T) -> &'static T { x }

#[rustc_promotable]
#[stable(feature = "a", since = "1.0.0")]
#[rustc_const_stable(feature = "a", since = "1.0.0")]
pub const fn new_string() -> String {
    String::new()
}

#[rustc_promotable]
#[stable(feature = "a", since = "1.0.0")]
#[rustc_const_stable(feature = "a", since = "1.0.0")]
pub const fn new_manually_drop<T>(t: T) -> std::mem::ManuallyDrop<T>  {
    std::mem::ManuallyDrop::new(t)
}


const C: () = {
    let _: &'static _ = &id(&new_string());
// { dg-error ".E0493." "" { target *-*-* } .-1 }
};

const _: () = {
    let _: &'static _ = &new_manually_drop(new_string());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
};

fn main() {
    let _: &'static _ = &id(&new_string());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
// { dg-error ".E0716." "" { target *-*-* } .-2 }

    let _: &'static _ = &new_manually_drop(new_string());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
}

