struct Data(String);

impl Iterator for Data {
    type Item = &str;
// { dg-error "" "" { target *-*-* } .-1 }

    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.0)
    }
}

trait Bar {
    type Item;
    fn poke(&mut self, item: Self::Item);
}

impl Bar for usize {
    type Item = &usize;
// { dg-error "" "" { target *-*-* } .-1 }

    fn poke(&mut self, item: Self::Item) {
        self += *item;
    }
}

impl Bar for isize {
    type Item<'a> = &'a isize;
// { dg-error ".E0195." "" { target *-*-* } .-1 }

    fn poke(&mut self, item: Self::Item) {
        self += *item;
    }
}

fn main() {}

