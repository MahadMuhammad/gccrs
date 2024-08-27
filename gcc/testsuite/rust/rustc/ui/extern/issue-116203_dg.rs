extern "C" {
    thread_local! {
      static FOO: u32 = 0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    }
}

macro_rules! hello {
    ($name:ident) => {
        const $name: () = ();
    };
}

extern "C" {
    hello! { yes }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

