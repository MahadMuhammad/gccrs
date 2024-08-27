trait Trait {
    type RefTarget;
}

impl Trait for ()
where
    Missing: Trait,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
    type RefTarget = ();
}

struct Other {
    data: <() as Trait>::RefTarget,
}

fn main() {
    unsafe {
        std::mem::transmute::<Option<()>, Option<&Other>>(None);
// { dg-error ".E0512." "" { target *-*-* } .-1 }
    }
}

