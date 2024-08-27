//@ run-rustfix

fn foo<N>(_x: N) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }
// { dg-note "" "" { target *-*-* } .-9 }
// { dg-note "" "" { target *-*-* } .-10 }
// { dg-note "" "" { target *-*-* } .-11 }
// { dg-note "" "" { target *-*-* } .-12 }
// { dg-note "" "" { target *-*-* } .-13 }
// { dg-note "" "" { target *-*-* } .-14 }
// { dg-note "" "" { target *-*-* } .-15 }
// { dg-note "" "" { target *-*-* } .-16 }
// { dg-note "" "" { target *-*-* } .-17 }
// { dg-note "" "" { target *-*-* } .-18 }
// { dg-note "" "" { target *-*-* } .-19 }
// { dg-note "" "" { target *-*-* } .-20 }
// { dg-note "" "" { target *-*-* } .-21 }
// { dg-note "" "" { target *-*-* } .-22 }

fn main() {
    foo::<isize>(42_usize);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_u64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_u32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_u16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_u8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_isize);
    foo::<isize>(42_i64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_i32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_i16);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42_i8);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42.0_f64);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
    foo::<isize>(42.0_f32);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
}

