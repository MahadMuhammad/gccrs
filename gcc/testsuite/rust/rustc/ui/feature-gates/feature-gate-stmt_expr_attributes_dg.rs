const X: i32 = #[allow(dead_code)] 8;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

const Y: i32 =
    /// foo
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    8;

const Z: i32 = {
    //! foo
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    8
};

fn main() {}

