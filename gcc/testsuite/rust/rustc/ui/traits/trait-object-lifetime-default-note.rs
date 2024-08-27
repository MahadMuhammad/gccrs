trait A {}

impl<T> A for T {}

fn main() {
    let local = 0; // { dg-note "" "" { target *-*-* } }
    let r = &local; // { dg-error ".E0597." "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    require_box(Box::new(r));
// { dg-note "" "" { target *-*-* } .-1 }

    let _ = 0;
} // { dg-note "" "" { target *-*-* } }

fn require_box(_a: Box<dyn A>) {}

