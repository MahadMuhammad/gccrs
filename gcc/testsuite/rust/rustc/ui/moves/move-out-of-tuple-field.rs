struct Foo(Box<isize>);



fn main() {
    let x: (Box<_>,) = (Box::new(1),);
    let y = x.0;
    let z = x.0; // { dg-error ".E0382." "" { target *-*-* } }

    let x = Foo(Box::new(1));
    let y = x.0;
    let z = x.0; // { dg-error ".E0382." "" { target *-*-* } }
}

