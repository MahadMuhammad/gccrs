//@ run-rustfix

#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;

fn test1() {
    let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn test2() {
    let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    println!();
}

macro_rules! mac {
    () => {
        fn test3() {
            let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
            println!();
        }
    };
}

struct A;

impl A {
    fn test4(&self) {
        let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }

    fn test5(&self) {
        let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
        println!();
    }
}

fn main() {
    let mut _file = File::create("foo.txt")?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    mac!();
}

