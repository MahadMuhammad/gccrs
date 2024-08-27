trait Hello {
    fn example(val: ());
}

struct Test1(i32);

impl Hello for Test1 {
    fn example(&self, input: &i32) {
// { dg-error ".E0185." "" { target *-*-* } .-1 }
        *input = self.0;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
    }
}

fn main() {}

