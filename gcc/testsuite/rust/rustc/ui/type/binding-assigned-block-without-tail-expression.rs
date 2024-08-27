struct S;
fn main() {
    let x = {
        println!("foo");
        42;
    };
    let y = {};
    let z = {
        "hi";
    };
    let s = {
        S;
    };
    println!("{}", x); // { dg-error ".E0277." "" { target *-*-* } }
    println!("{}", y); // { dg-error ".E0277." "" { target *-*-* } }
    println!("{}", z); // { dg-error ".E0277." "" { target *-*-* } }
    println!("{}", s); // { dg-error ".E0277." "" { target *-*-* } }
    let _: i32 = x; // { dg-error ".E0308." "" { target *-*-* } }
    let _: i32 = y; // { dg-error ".E0308." "" { target *-*-* } }
    let _: i32 = z; // { dg-error ".E0308." "" { target *-*-* } }
    let _: i32 = s; // { dg-error ".E0308." "" { target *-*-* } }
}

