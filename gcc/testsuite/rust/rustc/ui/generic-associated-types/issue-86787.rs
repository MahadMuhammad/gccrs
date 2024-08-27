//@ check-fail

enum Either<L, R> {
    Left(L),
    Right(R),
}

pub trait HasChildrenOf {
    type T;
    type TRef<'a>;
// { dg-error "" "" { target *-*-* } .-1 }

    fn ref_children<'a>(&'a self) -> Vec<Self::TRef<'a>>;
    fn take_children(self) -> Vec<Self::T>;
}

impl<Left, Right> HasChildrenOf for Either<Left, Right>
where
    Left: HasChildrenOf,
    Right: HasChildrenOf,
{
    type T = Either<Left::T, Right::T>;
    type TRef<'a> = Either<&'a Left::T, &'a Right::T>
    where
        <Left as HasChildrenOf>::T: 'a, // { dg-error ".E0276." "" { target *-*-* } }
        <Right as HasChildrenOf>::T: 'a; // { dg-error ".E0276." "" { target *-*-* } }

    fn ref_children<'a>(&'a self) -> Vec<Self::TRef<'a>> {
        todo!()
    }

    fn take_children(self) -> Vec<Self::T> {
        todo!()
    }
}

fn main() {}

