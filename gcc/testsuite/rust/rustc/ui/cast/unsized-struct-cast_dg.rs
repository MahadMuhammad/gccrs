pub struct Data([u8]);

fn main(){
    const _: *const Data = &[] as *const Data;
// { dg-error ".E0606." "" { target *-*-* } .-1 }
}

