#![feature(let_chains)]

fn main() {
    let opt = Some(1i32);

    let Some(n) = opt else {
        return;
    };
    let Some(n) = opt && n == 1 else {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
        return;
    };
    let Some(n) = opt && let another = n else {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
// { dg-error ".E0308." "" { target *-*-* } .-4 }
        return;
    };

    if let Some(n) = opt else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };
    if let Some(n) = opt && n == 1 else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };
    if let Some(n) = opt && let another = n else {
// { dg-error "" "" { target *-*-* } .-1 }
        return;
    };

    {
        while let Some(n) = opt else {
// { dg-error "" "" { target *-*-* } .-1 }
            return;
        };
    }
    {
        while let Some(n) = opt && n == 1 else {
// { dg-error "" "" { target *-*-* } .-1 }
            return;
        };
    }
    {
        while let Some(n) = opt && let another = n else {
// { dg-error "" "" { target *-*-* } .-1 }
            return;
        };
    }
}

