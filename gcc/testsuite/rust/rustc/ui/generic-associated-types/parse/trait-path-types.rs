trait X {
    type Y<'a>;
}

const _: () = {
  fn f<'a>(arg : Box<dyn X< [u8; 1] = u32>>) {}
// { dg-error "" "" { target *-*-* } .-1 }
};

const _: () = {
  fn f1<'a>(arg : Box<dyn X<(Y<'a>) = &'a ()>>) {}
// { dg-error "" "" { target *-*-* } .-1 }
};

const _: () = {
  fn f1<'a>(arg : Box<dyn X< 'a = u32 >>) {}
// { dg-error "" "" { target *-*-* } .-1 }
};

fn main() {}

