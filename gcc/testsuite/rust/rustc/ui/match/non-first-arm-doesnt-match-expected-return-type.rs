#![allow(unused)]

fn test(shouldwe: Option<u32>, shouldwe2: Option<u32>) -> u32 {
// { dg-note "" "" { target *-*-* } .-1 }
    match shouldwe {
        Some(val) => {
            match shouldwe2 {
                Some(val) => {
                    return val;
                }
                None => (), // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
            }
        }
        None => return 12,
    }
}

fn main() {
    println!("returned {}", test(None, Some(5)));
}

