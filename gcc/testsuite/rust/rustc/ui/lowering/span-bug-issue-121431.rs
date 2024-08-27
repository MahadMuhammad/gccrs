fn bug<T>() -> impl CallbackMarker< Item = [(); { |_: &mut ()| 3; 4 }] > {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }

fn main() {}

