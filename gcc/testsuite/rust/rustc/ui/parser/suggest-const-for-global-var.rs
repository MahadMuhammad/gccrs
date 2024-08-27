let X: i32 = 12;
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    println!("{}", X);
}

