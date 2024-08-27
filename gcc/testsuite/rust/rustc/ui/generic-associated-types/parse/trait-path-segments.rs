const _: () = {
    trait X {
        type Y<'a>;
    }

    fn f1<'a>(arg : Box<dyn X<X::Y = u32>>) {}
// { dg-error "" "" { target *-*-* } .-1 }
  };

const _: () = {
    trait X {
        type Y<'a>;
    }

    trait Z {}

    impl<T : X<<Self as X>::Y<'a> = &'a u32>> Z for T {}
// { dg-error "" "" { target *-*-* } .-1 }
};

const _: () = {
    trait X {
      type Y<'a>;
    }

    trait Z {}

    impl<T : X<X::Y<'a> = &'a u32>> Z for T {}
// { dg-error "" "" { target *-*-* } .-1 }
};

fn main() {}

