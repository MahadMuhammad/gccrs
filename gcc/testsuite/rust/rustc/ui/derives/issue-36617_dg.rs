#![derive(Copy)]
// { dg-error "" "" { target *-*-* } .-1 }

#![test]
// { dg-error "" "" { target *-*-* } .-1 }

#![test_case]
// { dg-error "" "" { target *-*-* } .-1 }

#![bench]
// { dg-error "" "" { target *-*-* } .-1 }

#![global_allocator]
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

