// Checks what happens when `public` is used instead of the correct, `pub`
// Won't give help message for this case

public let x = 1;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() { }

