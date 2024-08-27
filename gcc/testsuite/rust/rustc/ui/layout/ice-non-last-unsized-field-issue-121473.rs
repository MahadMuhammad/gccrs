// Regression test for #121473
// Checks that no ICE occurs when `size_of`
// is applied to a struct that has an unsized
// field which is not its last field

use std::mem::size_of;

pub struct BadStruct {
    pub field1: i32,
    pub field2: str, // Unsized field that is not the last field
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    pub field3: [u8; 16],
}

enum BadEnum1 {
    Variant1 {
        field1: i32,
        field2: str, // Unsized
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        field3: [u8; 16],
    },
}

enum BadEnum2 {
    Variant1(
        i32,
        str, // Unsized
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        [u8; 16]
    ),
}

enum BadEnumMultiVariant {
    Variant1(i32),
    Variant2 {
        field1: i32,
        field2: str, // Unsized
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        field3: [u8; 16],
    },
    Variant3
}

union BadUnion {
    field1: i32,
    field2: str, // Unsized
// { dg-error ".E0740." "" { target *-*-* } .-1 }
// { dg-error ".E0740." "" { target *-*-* } .-2 }
    field3: [u8; 16],
}

// Used to test that projection type fields that normalize
// to a sized type do not cause problems
struct StructWithProjections<'a>
{
    field1: <&'a [i32] as IntoIterator>::IntoIter,
    field2: i32
}

pub fn main() {
    let _a = &size_of::<BadStruct>();
    assert_eq!(size_of::<BadStruct>(), 21);

    let _a = &size_of::<BadEnum1>();
    assert_eq!(size_of::<BadEnum1>(), 21);

    let _a = &size_of::<BadEnum2>();
    assert_eq!(size_of::<BadEnum2>(), 21);

    let _a = &size_of::<BadEnumMultiVariant>();
    assert_eq!(size_of::<BadEnumMultiVariant>(), 21);

    let _a = &size_of::<BadUnion>();
    assert_eq!(size_of::<BadUnion>(), 21);

    let _a = &size_of::<StructWithProjections>();
    assert_eq!(size_of::<StructWithProjections>(), 21);
    let _a = StructWithProjections { field1: [1, 3].iter(), field2: 3 };
}

