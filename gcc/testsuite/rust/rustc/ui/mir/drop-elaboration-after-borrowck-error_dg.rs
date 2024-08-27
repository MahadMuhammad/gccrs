// Regression test for issue 81708 and issue 91816 where running a drop
// elaboration on a MIR which failed borrowck lead to an ICE.

static A: () = {
    let a: [String; 1];
// { dg-error ".E0493." "" { target *-*-* } .-1 }
    a[0] = String::new();
// { dg-error ".E0493." "" { target *-*-* } .-1 }
};

struct B<T>([T; 1]);

impl<T> B<T> {
    pub const fn f(mut self, other: T) -> Self {
        let _this = self;
// { dg-error ".E0493." "" { target *-*-* } .-1 }
        self.0[0] = other;
// { dg-error ".E0493." "" { target *-*-* } .-1 }
        self
    }
}

fn main() {}

