fn foo() {
    let mut foo = [1, 2, 3, 4];
    let a = &mut foo[2];
    let b = &mut foo[3]; // { dg-error ".E0499." "" { target *-*-* } }
    *a = 5;
    *b = 6;
    println!("{:?} {:?}", a, b);
}

fn bar() {
    let mut foo = [1,2,3,4];
    let a = &mut foo[..2];
    let b = &mut foo[2..]; // { dg-error ".E0499." "" { target *-*-* } }
    a[0] = 5;
    b[0] = 6;
    println!("{:?} {:?}", a, b);
}

fn baz() {
    let mut foo = [1,2,3,4];
    let a = &foo[..2];
    let b = &mut foo[2..]; // { dg-error ".E0502." "" { target *-*-* } }
    b[0] = 6;
    println!("{:?} {:?}", a, b);
}

fn qux() {
    let mut foo = [1,2,3,4];
    let a = &mut foo[..2];
    let b = &foo[2..]; // { dg-error ".E0502." "" { target *-*-* } }
    a[0] = 5;
    println!("{:?} {:?}", a, b);
}

fn bad() {
    let mut foo = [1,2,3,4];
    let a = &foo[1];
    let b = &mut foo[2]; // { dg-error ".E0502." "" { target *-*-* } }
    *b = 6;
    println!("{:?} {:?}", a, b);
}

fn bat() {
    let mut foo = [1,2,3,4];
    let a = &mut foo[1];
    let b = &foo[2]; // { dg-error ".E0502." "" { target *-*-* } }
    *a = 5;
    println!("{:?} {:?}", a, b);
}

fn ang() {
    let mut foo = [1,2,3,4];
    let a = &mut foo[0..];
    let b = &foo[0..]; // { dg-error ".E0502." "" { target *-*-* } }
    a[0] = 5;
    println!("{:?} {:?}", a, b);
}

fn main() {
    foo();
    bar();
}

