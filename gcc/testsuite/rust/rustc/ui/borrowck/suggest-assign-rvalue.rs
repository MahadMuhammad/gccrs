#![allow(dead_code)]
#![feature(never_type)]

#[derive(Debug, Default)]
struct Demo {}

#[derive(Debug)]
struct DemoNoDef {}

fn apple(_: u32) {}

fn banana() {
    let chaenomeles;
    apple(chaenomeles);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
}

fn main() {
    let my_bool: bool = bool::default();
    println!("my_bool: {}", my_bool);

    let my_float: f32;
    println!("my_float: {}", my_float);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
    let demo: Demo;
    println!("demo: {:?}", demo);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let demo_no: DemoNoDef;
    println!("demo_no: {:?}", demo_no);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let arr: [i32; 5];
    println!("arr: {:?}", arr);
// { dg-error ".E0381." "" { target *-*-* } .-1 }
    let foo: Vec<&str>;
    println!("foo: {:?}", foo);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let my_string: String;
    println!("my_string: {}", my_string);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let my_int: &i32;
    println!("my_int: {}", *my_int);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let hello: &str;
    println!("hello: {}", hello);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    let never: !;
    println!("never: {}", never);
// { dg-error ".E0381." "" { target *-*-* } .-1 }

    banana();
}

