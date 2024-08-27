pub struct Struct {
    a: Vec<Struct>,
}

impl Struct {
    pub fn test(&self) {
        if let [Struct { a: [] }] = &self.a {
// { dg-error ".E0529." "" { target *-*-* } .-1 }
// { dg-error ".E0529." "" { target *-*-* } .-2 }
            println!("matches!")
        }

        if let [Struct { a: [] }] = &self.a[..] {
// { dg-error ".E0529." "" { target *-*-* } .-1 }
            println!("matches!")
        }
    }
}

fn main() {}

