use std::cell::Cell;
use std::rc::Rc;

// Fast path, main can see the concrete type returned.
fn before() -> impl Fn(i32) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    let p = Rc::new(Cell::new(0));
    move |x| p.set(x) // { dg-note "" "" { target *-*-* } }
}

fn send<T: Send>(_: T) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }

fn main() {
    send(before());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }

    send(after());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
}

// Deferred path, main has to wait until typeck finishes,
// to check if the return type of after is Send.
fn after() -> impl Fn(i32) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    let p = Rc::new(Cell::new(0));
    move |x| p.set(x) // { dg-note "" "" { target *-*-* } }
}

