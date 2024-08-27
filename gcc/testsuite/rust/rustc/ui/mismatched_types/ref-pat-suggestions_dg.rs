//@ run-rustfix

fn _f0(&_a: u32) {} // { dg-error ".E0308." "" { target *-*-* } }
fn _f1(&mut _a: u32) {} // { dg-error ".E0308." "" { target *-*-* } }
fn _f2(&&_a: &u32) {} // { dg-error ".E0308." "" { target *-*-* } }
fn _f3(&mut &_a: &mut u32) {} // { dg-error ".E0308." "" { target *-*-* } }
fn _f4(&&mut _a: &u32) {} // { dg-error ".E0308." "" { target *-*-* } }
fn _f5(&mut &mut _a: &mut u32) {} // { dg-error ".E0308." "" { target *-*-* } }

fn main() {
    let _: fn(u32) = |&_a| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _: fn(u32) = |&mut _a| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _: fn(&u32) = |&&_a| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _: fn(&mut u32) = |&mut &_a| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _: fn(&u32) = |&&mut _a| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _: fn(&mut u32) = |&mut &mut _a| (); // { dg-error ".E0308." "" { target *-*-* } }

    let _ = |&_a: u32| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _ = |&mut _a: u32| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _ = |&&_a: &u32| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _ = |&mut &_a: &mut u32| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _ = |&&mut _a: &u32| (); // { dg-error ".E0308." "" { target *-*-* } }
    let _ = |&mut &mut _a: &mut u32| (); // { dg-error ".E0308." "" { target *-*-* } }

    #[allow(unused_mut)]
    {
        struct S(u8);

        let &mut _a = 0; // { dg-error ".E0308." "" { target *-*-* } }
        let S(&mut _b) = S(0); // { dg-error ".E0308." "" { target *-*-* } }
        let (&mut _c,) = (0,); // { dg-error ".E0308." "" { target *-*-* } }

        match 0 {
            &mut _d => {} // { dg-error ".E0308." "" { target *-*-* } }
        }
    }
}

