//@ run-rustfix
mod test {
    public const X: i32 = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    println!("{}", test::X);
}

