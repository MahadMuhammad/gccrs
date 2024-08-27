// { dg-additional-options "-frust-edition= 2021" }
#![allow(incomplete_features)]
#![feature(ref_pat_eat_one_layer_2024)]
pub fn main() {
    if let Some(Some(&x)) = &Some(&Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(Some(&x)) = &Some(Some(&0)) {
        let _: &u32 = x;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
    if let Some(Some(&&x)) = &Some(Some(&0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(&Some(x)) = &Some(Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(Some(&mut x)) = &mut Some(&mut Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(Some(&x)) = &Some(&Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(&mut Some(&x)) = &Some(&mut Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
    if let Some(&Some(&mut x)) = &mut Some(&Some(0)) {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        let _: u32 = x;
    }
}

