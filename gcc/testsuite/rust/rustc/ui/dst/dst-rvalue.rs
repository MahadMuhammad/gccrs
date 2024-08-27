// Check that dynamically sized rvalues are forbidden

pub fn main() {
    let _x: Box<str> = Box::new(*"hello world");
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    let array: &[isize] = &[1, 2, 3];
    let _x: Box<[isize]> = Box::new(*array);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

