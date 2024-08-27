fn dup(x: Box<isize>) -> Box<(Box<isize>,Box<isize>)> {


    Box::new((x, x))
// { dg-error ".E0382." "" { target *-*-* } .-1 }
}

fn main() {
    dup(Box::new(3));
}

