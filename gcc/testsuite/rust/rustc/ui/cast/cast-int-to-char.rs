fn foo<T>(_t: T) {}

fn main() {
    foo::<u32>('0');  // { dg-error ".E0308." "" { target *-*-* } }
    foo::<i32>('0');  // { dg-error ".E0308." "" { target *-*-* } }
    foo::<u64>('0');  // { dg-error ".E0308." "" { target *-*-* } }
    foo::<i64>('0');  // { dg-error ".E0308." "" { target *-*-* } }
    foo::<char>(0u32);  // { dg-error ".E0308." "" { target *-*-* } }
}

