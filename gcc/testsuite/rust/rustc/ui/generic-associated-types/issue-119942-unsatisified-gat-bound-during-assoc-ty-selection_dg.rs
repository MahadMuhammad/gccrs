use std::ops::Deref;

trait PointerFamily {
    type Pointer<T>: Deref<Target = T>;
}

struct RcFamily;

impl PointerFamily for RcFamily {
    type Pointer<T> = dyn Deref<Target = T>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

enum Node<T, P: PointerFamily> {
    Cons(T, P::Pointer<Node<T, P>>),
    Nil,
}

type RcNode<T> = Node<T, RcFamily>;

impl<T, P: PointerFamily> Node<T, P>
where
    P::Pointer<Node<T, P>>: Sized,
{
    fn new() -> P::Pointer<Self> {
        todo!()
    }
}

fn main() {
    let mut list = RcNode::<i32>::new();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

