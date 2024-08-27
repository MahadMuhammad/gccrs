fn main() {
  let _ = fix(|_: &dyn Fn()| {});
}

fn fix<F: Fn(G), G: Fn()>(f: F) -> impl Fn() {
  move || f(fix(&f))
// { dg-error ".E0792." "" { target *-*-* } .-1 }
// { dg-error ".E0792." "" { target *-*-* } .-2 }
}

