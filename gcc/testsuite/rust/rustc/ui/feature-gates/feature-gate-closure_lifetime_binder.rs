fn main() {
    for<> || -> () {};
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    for<'a> || -> () {};
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    for<'a, 'b> |_: &'a ()| -> () {};
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

