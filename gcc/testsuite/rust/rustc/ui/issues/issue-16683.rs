trait T<'a> {
    fn a(&'a self) -> &'a bool;
    fn b(&self) {
        self.a();
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

