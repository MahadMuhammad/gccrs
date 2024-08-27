fn main() {
    [(); &(&'static: loop { |x| {}; }) as *const _ as usize]
// { dg-error ".E0282." "" { target *-*-* } .-1 }
// { dg-error ".E0282." "" { target *-*-* } .-2 }
}

