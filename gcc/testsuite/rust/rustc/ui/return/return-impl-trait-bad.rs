trait Trait {}
impl Trait for () {}

fn bad_echo<T>(_t: T) -> T {
    "this should not suggest impl Trait" // { dg-error ".E0308." "" { target *-*-* } }
}

fn bad_echo_2<T: Trait>(_t: T) -> T {
    "this will not suggest it, because that would probably be wrong" // { dg-error ".E0308." "" { target *-*-* } }
}

fn other_bounds_bad<T>() -> T
where
    T: Send,
    Option<T>: Send,
{
    "don't suggest this, because Option<T> places additional constraints" // { dg-error ".E0308." "" { target *-*-* } }
}

// FIXME: implement this check
trait GenericTrait<T> {}

fn used_in_trait<T>() -> T
where
    T: Send,
    (): GenericTrait<T>,
{
    "don't suggest this, because the generic param is used in the bound." // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {}

