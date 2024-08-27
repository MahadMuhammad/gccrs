struct Foo<'c, 'd>(&'c (), &'d ());

impl<'c, 'd> Foo<'c, 'd> {
    fn acc(&mut self, _bar: &Bar) -> &'d () {
        todo!()
    }
}

struct Bar;

impl<'a> Bar {
    fn boom(&self, foo: &mut Foo<'_, '_, 'a>) -> Result<(), &'a ()> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
        self.bar().map_err(|()| foo.acc(self))?;
// { dg-error ".E0621." "" { target *-*-* } .-1 }
        Ok(())
    }
    fn bar(&self) -> Result<(), &'a ()> {
        todo!()
    }
}

fn main() {}

