trait Trait<const N: Trait = bar> {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
// { dg-error ".E0038." "" { target *-*-* } .-6 }
// { dg-warning ".E0038." "" { target *-*-* } .-7 }
// { dg-warning ".E0038." "" { target *-*-* } .-8 }
// { dg-warning ".E0038." "" { target *-*-* } .-9 }
// { dg-warning ".E0038." "" { target *-*-* } .-10 }
    fn fnc<const N: Trait = u32>(&self) -> Trait {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }
// { dg-error ".E0038." "" { target *-*-* } .-4 }
// { dg-error ".E0038." "" { target *-*-* } .-5 }
// { dg-error ".E0038." "" { target *-*-* } .-6 }
// { dg-warning ".E0038." "" { target *-*-* } .-7 }
// { dg-warning ".E0038." "" { target *-*-* } .-8 }
// { dg-warning ".E0038." "" { target *-*-* } .-9 }
// { dg-warning ".E0038." "" { target *-*-* } .-10 }
// { dg-warning ".E0038." "" { target *-*-* } .-11 }
// { dg-warning ".E0038." "" { target *-*-* } .-12 }
        bar
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

fn main() {}

