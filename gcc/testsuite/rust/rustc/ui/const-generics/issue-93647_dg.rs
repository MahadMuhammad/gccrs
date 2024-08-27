struct X<const N: usize = {
    (||1usize)()
// { dg-error ".E0015." "" { target *-*-* } .-1 }
}>;

fn main() {}

