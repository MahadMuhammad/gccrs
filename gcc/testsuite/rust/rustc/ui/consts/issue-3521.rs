//@ run-rustfix
fn main() {
    #[allow(non_upper_case_globals)]
    let foo: isize = 100;

    #[derive(Debug)]
    enum Stuff {
        Bar = foo
// { dg-error ".E0435." "" { target *-*-* } .-1 }
    }

    println!("{:?}", Stuff::Bar);
}

