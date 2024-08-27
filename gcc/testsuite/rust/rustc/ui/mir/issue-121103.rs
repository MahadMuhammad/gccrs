fn main(_: <lib2::GenericType<42> as lib2::TypeFn>::Output) {}
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }

