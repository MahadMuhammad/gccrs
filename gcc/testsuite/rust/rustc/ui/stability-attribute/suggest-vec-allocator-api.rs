fn main() {
    let _: Vec<u8, _> = vec![]; // { dg-error ".E0658." "" { target *-*-* } }
    #[rustfmt::skip]
    let _: Vec<
        String,
        _> = vec![]; // { dg-error ".E0658." "" { target *-*-* } }
    let _ = Vec::<u16, _>::new(); // { dg-error ".E0658." "" { target *-*-* } }
    let _boxed: Box<u32, _> = Box::new(10); // { dg-error ".E0658." "" { target *-*-* } }
}

