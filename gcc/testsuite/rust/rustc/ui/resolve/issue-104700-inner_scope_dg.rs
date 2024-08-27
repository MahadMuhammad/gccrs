fn main() {
    let foo = 1;
    {
        let bar = 2;
        let test_func = |x| x > 3;
    }
    if bar == 2 { // { dg-error ".E0425." "" { target *-*-* } }
        println!("yes");
    }
    test_func(1); // { dg-error ".E0425." "" { target *-*-* } }
}

