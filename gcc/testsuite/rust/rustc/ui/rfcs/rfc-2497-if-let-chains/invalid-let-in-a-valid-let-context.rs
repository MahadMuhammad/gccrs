#![feature(let_chains)]

fn main() {
    let _opt = Some(1i32);

    #[cfg(FALSE)]
    {
        let _ = &&let Some(x) = Some(42);
// { dg-error "" "" { target *-*-* } .-1 }
    }
    #[cfg(FALSE)]
    {
        if let Some(elem) = _opt && [1, 2, 3][let _ = &&let Some(x) = Some(42)] = 1 {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
            true
        }
    }

    #[cfg(FALSE)]
    {
        if let Some(elem) = _opt && {
            [1, 2, 3][let _ = ()];
// { dg-error "" "" { target *-*-* } .-1 }
            true
        } {
        }
    }

    #[cfg(FALSE)]
    {
        if let Some(elem) = _opt && [1, 2, 3][let _ = ()] = 1 {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
            true
        }
    }
    #[cfg(FALSE)]
    {
        if let a = 1 && {
            let x = let y = 1;
// { dg-error "" "" { target *-*-* } .-1 }
        } {
        }
    }
}

