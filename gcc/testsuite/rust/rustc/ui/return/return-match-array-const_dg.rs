fn main() {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    [(); return match 0 { n => n }];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }

    [(); return match 0 { 0 => 0 }];
// { dg-error ".E0572." "" { target *-*-* } .-1 }
// { dg-note ".E0572." "" { target *-*-* } .-2 }

    [(); return match () { 'a' => 0, _ => 0 }];
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
// { dg-note ".E0308." "" { target *-*-* } .-5 }
}

