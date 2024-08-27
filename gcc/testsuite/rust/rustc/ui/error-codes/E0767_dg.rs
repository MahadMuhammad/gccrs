fn main() {
    'a: loop {
        || {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
            loop { break 'a; } // { dg-error ".E0767." "" { target *-*-* } }
        }
    }
}

