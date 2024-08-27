//@ run-rustfix

import std::{
// { dg-error "" "" { target *-*-* } .-1 }
    io::Write,
    rc::Rc,
};

require std::time::Duration;
// { dg-error "" "" { target *-*-* } .-1 }

include std::time::Instant;
// { dg-error "" "" { target *-*-* } .-1 }

pub using std::io;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    let x = Rc::new(1);
    let _ = write!(io::stdout(), "{:?}", x);
    let _ = Duration::new(5, 0);
    let _ = Instant::now();
}

