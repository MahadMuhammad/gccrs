// { dg-additional-options "-frust-edition=2021" }

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let mut p = Point {x: 1, y: 2 };

    let y = &mut p.y;
    let mut c = || {
// { dg-error ".E0499." "" { target *-*-* } .-1 }
       let x = &mut p.x;
       println!("{:?}", p);
    };
    c();
    *y+=1;
}

