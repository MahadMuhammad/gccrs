// rust-lang/rust#83309: The compiler tries to suggest potential
// methods that return `&mut` items. However, when it doesn't
// find such methods, it still tries to add suggestions
// which then fails an assertion later because there was
// no suggestions to make.


fn main() {
    for v in Query.iter_mut() {
// { dg-note "" "" { target *-*-* } .-1 }
        *v -= 1;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
// { dg-note ".E0594." "" { target *-*-* } .-2 }
    }
}

pub struct Query;
pub struct QueryIter<'a>(&'a i32);

impl Query {
    pub fn iter_mut<'a>(&'a mut self) -> QueryIter<'a> {
        todo!();
    }
}

impl<'a> Iterator for QueryIter<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }
}

