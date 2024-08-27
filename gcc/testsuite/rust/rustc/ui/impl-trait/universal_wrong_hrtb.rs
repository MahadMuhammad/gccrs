trait Trait<'a> {
    type Assoc;
}

fn test_argument_position(x: impl for<'a> Trait<'a, Assoc = impl Copy + 'a>) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

