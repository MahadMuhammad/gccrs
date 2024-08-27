pub type Archived<T> = <m::Alias as n::Trait>::Archived;
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }

fn main() {}

