//@ normalize-stderr-test: "DefId\(.+?\)" -> "DefId(..)"

#![feature(rustc_attrs)]

#[rustc_dump_predicates]
trait Trait<T>: Iterator<Item: Copy>
// { dg-error "" "" { target *-*-* } .-1 }
where
    String: From<T>
{
    #[rustc_dump_predicates]
    #[rustc_dump_item_bounds]
    type Assoc<P: Eq>: std::ops::Deref<Target = ()>
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    where
        Self::Assoc<()>: Copy;
}

fn main() {}

