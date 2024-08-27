trait Foo: std::fmt::Debug {}

fn print_foos(foos: impl Iterator<Item = dyn Foo>) {
    foos.for_each(|foo| { // { dg-error ".E0277." "" { target *-*-* } }
        println!("{:?}", foo); // { dg-error ".E0277." "" { target *-*-* } }
    });
}

fn main() {}

