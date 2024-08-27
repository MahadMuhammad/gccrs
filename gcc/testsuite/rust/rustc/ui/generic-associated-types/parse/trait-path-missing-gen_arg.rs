trait X {
    type Y<'a>;
}

const _: () = {
  fn f1<'a>(arg : Box<dyn X< : 32 >>) {}
// { dg-error "" "" { target *-*-* } .-1 }
};

const _: () = {
  fn f1<'a>(arg : Box<dyn X< = 32 >>) {}
// { dg-error "" "" { target *-*-* } .-1 }
};

fn main() {}

