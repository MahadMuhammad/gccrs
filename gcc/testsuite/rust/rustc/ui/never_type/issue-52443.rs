fn main() {
    [(); & { loop { continue } } ]; // { dg-error ".E0308." "" { target *-*-* } }

    [(); loop { break }]; // { dg-error ".E0308." "" { target *-*-* } }

    [(); {while true {break}; 0}];
// { dg-warning "" "" { target *-*-* } .-1 }

    [(); { for _ in 0usize.. {}; 0}];
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
// { dg-error ".E0015." "" { target *-*-* } .-3 }
// { dg-error ".E0015." "" { target *-*-* } .-4 }
}

