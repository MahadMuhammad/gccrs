//@ run-rustfix
#![allow(dead_code)]

 pub fn foo(x: &str) -> Result<(), Box<dyn std::error::Error>> {
     Err(format!("error: {x}"))
// { dg-error ".E0308." "" { target *-*-* } .-1 }
 }

 macro_rules! outer {
     ($x: expr) => {
         inner!($x)
     }
 }

 macro_rules! inner {
     ($x: expr) => {
         format!("error: {}", $x)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
     }
 }

 fn bar(x: &str) -> Result<(), Box<dyn std::error::Error>> {
     Err(outer!(x))
 }

 macro_rules! entire_fn_outer {
     () => {
         entire_fn!();
     }
 }

 macro_rules! entire_fn {
     () => {
         pub fn baz(x: &str) -> Result<(), Box<dyn std::error::Error>> {
             Err(format!("error: {x}"))
// { dg-error ".E0308." "" { target *-*-* } .-1 }
         }
     }
 }

 entire_fn_outer!();

macro_rules! nontrivial {
    ($x: expr) => {
        Err(format!("error: {}", $x))
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

pub fn qux(x: &str) -> Result<(), Box<dyn std::error::Error>> {
    nontrivial!(x)
}


fn main() {}

