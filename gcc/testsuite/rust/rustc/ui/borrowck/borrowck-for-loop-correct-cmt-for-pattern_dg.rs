// Issue #16205.



struct Foo {
    a: [Box<isize>; 3],
}

fn main() {
    let mut y = 1;
    let x = Some(&mut y);
    for &a in x.iter() {    // { dg-error ".E0507." "" { target *-*-* } }
    }

    let f = Foo {
        a: [Box::new(3), Box::new(4), Box::new(5)],
    };
    for &a in &f.a {  // { dg-error ".E0507." "" { target *-*-* } }
    }

    let x: Option<Box<_>> = Some(Box::new(1));
    for &a in x.iter() {    // { dg-error ".E0507." "" { target *-*-* } }
    }
}

