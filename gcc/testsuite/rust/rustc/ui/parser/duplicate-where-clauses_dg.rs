struct A where (): Sized where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

fn b() where (): Sized where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

enum C where (): Sized where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

struct D where (): Sized, where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

fn e() where (): Sized, where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

enum F where (): Sized, where (): Sized {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

