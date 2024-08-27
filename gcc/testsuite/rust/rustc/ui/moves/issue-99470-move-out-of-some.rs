fn main() {
    let x: &Option<Box<i32>> = &Some(Box::new(0));

    match x {
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        &Some(_y) => (),
        &None => (),
    }
}

