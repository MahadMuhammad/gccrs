struct MutType;

pub trait MutTrait {
    fn function(&mut self)
    where
        Self: Sized;
}

impl MutTrait for MutType {
    fn function(&mut self) {}
}

struct Type;

pub trait Trait {
    fn function(&self)
    where
        Self: Sized;
}

impl Trait for Type {
    fn function(&self) {}
}

fn main() {
    (&MutType as &dyn MutTrait).function();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    (&mut Type as &mut dyn Trait).function();
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

