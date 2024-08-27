const _: () = unsafe {
    let n = u32::MAX.count_ones();
    std::hint::assert_unchecked(n < 32); // { dg-error ".E0080." "" { target *-*-* } }
};

fn main() {}

