fn foo(x: Result<i32, ()>) -> Result<(), ()> {
    let y: u32 = x?;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    Ok(())
}

fn main() {}

