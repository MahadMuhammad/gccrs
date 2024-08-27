use std::mem::ManuallyDrop;

struct S1 { f: dyn Iterator<Item: Copy> }
// { dg-error "" "" { target *-*-* } .-1 }
struct S2 { f: Box<dyn Iterator<Item: Copy>> }
// { dg-error "" "" { target *-*-* } .-1 }
struct S3 { f: dyn Iterator<Item: 'static> }
// { dg-error "" "" { target *-*-* } .-1 }

enum E1 { V(dyn Iterator<Item: Copy>) }
// { dg-error "" "" { target *-*-* } .-1 }
enum E2 { V(Box<dyn Iterator<Item: Copy>>) }
// { dg-error "" "" { target *-*-* } .-1 }
enum E3 { V(dyn Iterator<Item: 'static>) }
// { dg-error "" "" { target *-*-* } .-1 }

union U1 { f: ManuallyDrop<dyn Iterator<Item: Copy>> }
// { dg-error "" "" { target *-*-* } .-1 }
union U2 { f: ManuallyDrop<Box<dyn Iterator<Item: Copy>>> }
// { dg-error "" "" { target *-*-* } .-1 }
union U3 { f: ManuallyDrop<dyn Iterator<Item: 'static>> }
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

