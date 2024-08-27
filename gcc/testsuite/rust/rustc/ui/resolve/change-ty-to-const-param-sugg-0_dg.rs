fn make<N: u32>() {}
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { help ".E0404." "" { target *-*-* } .-2 }

struct Array<N: usize>([bool; N]);
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
// { dg-error ".E0423." "" { target *-*-* } .-3 }

fn main() {}

