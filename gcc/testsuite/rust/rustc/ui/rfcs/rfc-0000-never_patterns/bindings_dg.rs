#![feature(never_patterns)]
#![allow(incomplete_features)]

#[derive(Copy, Clone)]
enum Void {}

fn main() {
    let x: Result<bool, &(u32, u32, Void)> = Ok(false);

    match x {
        Ok(_x) | Err(&!) => {}
    }
    let (Ok(_x) | Err(&!)) = x;

    match x {
        Ok(_) => {}
        Err(&(_a, _b, !)),
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
    match x {
        Ok(_ok) | Err(&(_a, _b, !)) => {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

fn void(void: Void) {
    let (_a | !) = void;
    let (! | _a) = void;
    let ((_a, _) | (_a, _ | !)) = (true, void);
    let (_a | (! | !,)) = (void,);
    let ((_a,) | (!,)) = (void,);

    let (_a, (! | !)) = (true, void);
// { dg-error "" "" { target *-*-* } .-1 }
    let (_a, (_b | !)) = (true, void);

    let _a @ ! = void;
// { dg-error "" "" { target *-*-* } .-1 }
    let _a @ (_b | !) = void;
    let (_a @ (), !) = ((), void);
// { dg-error "" "" { target *-*-* } .-1 }
    let (_a |
            (_b @ (_, !))) = (true, void);
// { dg-error "" "" { target *-*-* } .-1 }
}

