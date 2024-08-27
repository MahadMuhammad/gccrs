struct Bar<'a> {
    s: &'a String,
    // use wonky spaces to ensure we are creating the span correctly
    longer_name:   &   'a     Vec<u8>
}

impl<'a> Bar<'a> {
    fn f(&mut self) {
        self.s.push('x');
// { dg-error ".E0596." "" { target *-*-* } .-1 }

        self.longer_name.push(13);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    }
}

fn main() {}

