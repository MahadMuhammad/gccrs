// Parenthesised let "expressions" are not allowed in guards

#![feature(if_let_guard)]
#![feature(let_chains)]

#[cfg(FALSE)]
fn un_cfged() {
    match () {
        () if let 0 = 1 => {}
        () if (let 0 = 1) => {}
// { dg-error "" "" { target *-*-* } .-1 }
        () if (((let 0 = 1))) => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {
    match () {
        () if let 0 = 1 => {}
        () if (let 0 = 1) => {}
// { dg-error "" "" { target *-*-* } .-1 }
        () if (((let 0 = 1))) => {}
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

