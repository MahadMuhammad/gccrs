//@ run-rustfix

fn main() {
    demo = 1; // { dg-error ".E0425." "" { target *-*-* } }
    dbg!(demo); // { dg-error ".E0425." "" { target *-*-* } }

    x = "x"; // { dg-error ".E0425." "" { target *-*-* } }
    println!("x: {}", x); // { dg-error ".E0425." "" { target *-*-* } }

    let_some_variable = 6; // { dg-error ".E0425." "" { target *-*-* } }
    println!("some_variable: {}", some_variable); // { dg-error ".E0425." "" { target *-*-* } }

    letother_variable = 6; // { dg-error ".E0425." "" { target *-*-* } }
    println!("other_variable: {}", other_variable); // { dg-error ".E0425." "" { target *-*-* } }

    if x == "x" {
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        println!("x is 1");
    }

    y = 1 + 2; // { dg-error ".E0425." "" { target *-*-* } }
    println!("y: {}", y); // { dg-error ".E0425." "" { target *-*-* } }
}

