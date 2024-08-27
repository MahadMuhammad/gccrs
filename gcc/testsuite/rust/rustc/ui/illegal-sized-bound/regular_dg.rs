struct MutType;

pub trait MutTrait {
    fn function(&mut self)
    where
        Self: Sized;
// { dg-error "" "" { target *-*-* } .-1 }
}

impl MutTrait for MutType {
    fn function(&mut self) {}
}

struct Type;

pub trait Trait {
    fn function(&self)
    where
        Self: Sized;
// { dg-error "" "" { target *-*-* } .-1 }
}

impl Trait for Type {
    fn function(&self) {}
}

fn main() {
    (&mut MutType as &mut dyn MutTrait).function();
// { dg-error "" "" { target *-*-* } .-1 }
    (&Type as &dyn Trait).function();
// { dg-error "" "" { target *-*-* } .-1 }
}

