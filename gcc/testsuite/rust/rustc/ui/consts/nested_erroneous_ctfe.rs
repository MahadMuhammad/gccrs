fn main() {
    [9; || [9; []]];
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

