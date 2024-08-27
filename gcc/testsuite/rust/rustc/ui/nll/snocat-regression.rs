// Regression test from https://github.com/rust-lang/rust/pull/98109

pub fn negotiate<S>(link: S)
where
    for<'a> &'a S: 'a,
{
    || {
// { dg-error "" "" { target *-*-* } .-1 }
        //
        // FIXME(#98437). This regressed at some point and
        // probably should work.
        let _x = link;
    };
}

fn main() {}

