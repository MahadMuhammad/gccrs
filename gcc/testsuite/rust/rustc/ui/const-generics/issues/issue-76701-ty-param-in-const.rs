fn ty_param<T>() -> [u8; std::mem::size_of::<T>()] {
// { dg-error "" "" { target *-*-* } .-1 }
    todo!()
}

fn const_param<const N: usize>() -> [u8; N + 1] {
// { dg-error "" "" { target *-*-* } .-1 }
    todo!()
}

fn main() {}

