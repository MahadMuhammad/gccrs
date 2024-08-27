pub const fn id<T>(x: T) -> T { x }
pub const C: () = {
    let _: &'static _ = &String::new();
// { dg-error ".E0493." "" { target *-*-* } .-1 }
};

pub const _: () = {
    let _: &'static _ = &id(&String::new());
// { dg-error ".E0493." "" { target *-*-* } .-1 }
};

pub const _: () = {
    let _: &'static _ = &std::mem::ManuallyDrop::new(String::new());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
};

fn main() {
    let _: &'static _ = &String::new();
// { dg-error ".E0716." "" { target *-*-* } .-1 }

    let _: &'static _ = &id(&String::new());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
// { dg-error ".E0716." "" { target *-*-* } .-2 }

    let _: &'static _ = &std::mem::ManuallyDrop::new(String::new());
// { dg-error ".E0716." "" { target *-*-* } .-1 }
}

