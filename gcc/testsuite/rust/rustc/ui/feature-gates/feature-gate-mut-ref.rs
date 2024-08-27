fn main() {
    let mut ref x = 10; // { dg-error ".E0658." "" { target *-*-* } }
    x = &11;
    let ref mut y = 12;
    *y = 13;
    let mut ref mut z = 14; // { dg-error ".E0658." "" { target *-*-* } }
    z = &mut 15;

    #[cfg(FALSE)]
    let mut ref x = 10; // { dg-error ".E0658." "" { target *-*-* } }
    #[cfg(FALSE)]
    let mut ref mut y = 10; // { dg-error ".E0658." "" { target *-*-* } }
}

