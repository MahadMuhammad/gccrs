struct Struct<const N: usize = { Self; 10 }>;
// { dg-error ".E0735." "" { target *-*-* } .-1 }

enum Enum<const N: usize = { Self; 10 }> { }
// { dg-error ".E0735." "" { target *-*-* } .-1 }

union Union<const N: usize = { Self; 10 }> { not_empty: () }
// { dg-error ".E0735." "" { target *-*-* } .-1 }

fn main() {
    let _: Struct;
    let _: Enum;
    let _: Union;
}

