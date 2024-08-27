fn foo() {
    let a = 0;
    let b = 4;
    if a =< b { // { dg-error "" "" { target *-*-* } }
        println!("yay!");
    }
}

fn bar() {
    let a = 0;
    let b = 4;
    if a = <b { // { dg-error "" "" { target *-*-* } }
        println!("yay!");
    }
}

fn baz() {
    let a = 0;
    let b = 4;
    if a = < b { // { dg-error "" "" { target *-*-* } }
        println!("yay!");
    }
}

fn qux() {
    let a = 0;
    let b = 4;
    if a =< i32>::abs(-4) { // { dg-error ".E0308." "" { target *-*-* } }
        println!("yay!");
    }
}

fn main() {}

