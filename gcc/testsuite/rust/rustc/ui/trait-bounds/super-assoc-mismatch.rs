trait Super {
    type Assoc;
}
impl Super for () {
    type Assoc = u8;
}
trait Sub: Super<Assoc = u16> {}

trait BoundOnSelf: Sub {}
impl BoundOnSelf for () {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

trait BoundOnParam<T: Sub> {}
impl BoundOnParam<()> for () {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

trait BoundOnAssoc {
    type Assoc: Sub;
}
impl BoundOnAssoc for () {
    type Assoc = ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

trait BoundOnGat where Self::Assoc<u8>: Sub {
    type Assoc<T>;
}
impl BoundOnGat for u8 {
    type Assoc<T> = ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn trivial_bound() where (): Sub {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

// The following is an edge case where the unsatisfied projection predicate
// `<<u8 as MultiAssoc>::Assoc1<()> as SuperGeneric<u16>>::Assoc == <u8 as MultiAssoc>::Assoc2`
// contains both associated types of `MultiAssoc`. To suppress the error about the unsatisfied
// super projection, the error's span must be equal to the span of the unsatisfied trait error.
trait SuperGeneric<T> {
    type Assoc;
}
trait SubGeneric<T>: SuperGeneric<T, Assoc = T> {}
trait MultiAssoc
where
    Self::Assoc1<()>: SubGeneric<Self::Assoc2>
{
    type Assoc1<T>;
    type Assoc2;
}
impl SuperGeneric<u16> for () {
    type Assoc = u8;
}
impl MultiAssoc for u8 {
    type Assoc1<T> = ();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    type Assoc2 = u16;
}

fn main() {}

