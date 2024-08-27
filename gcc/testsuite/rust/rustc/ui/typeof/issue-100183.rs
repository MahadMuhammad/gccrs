struct Struct {
    y: (typeof("hey"),),
// { dg-error ".E0516." "" { target *-*-* } .-1 }
}

fn main() {}

