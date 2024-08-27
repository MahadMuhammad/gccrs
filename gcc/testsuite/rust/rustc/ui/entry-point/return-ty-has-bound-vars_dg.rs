// issue-119209

fn main<'a>(_: &'a i32) -> &'a () { &() } // { dg-error ".E0131." "" { target *-*-* } }

