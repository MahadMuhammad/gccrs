mod a {
    type Foo = u64;
    type Bar = u64;
}

mod b {
    type Foo = u64;
}

fn main() {
    let x: Foo = 100; // { dg-error ".E0412." "" { target *-*-* } }
    let y: Bar = 100; // { dg-error ".E0412." "" { target *-*-* } }
    println!("x: {}, y: {}", x, y);
}

