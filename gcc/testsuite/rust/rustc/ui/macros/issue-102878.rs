macro_rules!test{($l:expr,$_:r)=>({const:y y)}
// { dg-error "" "" { target *-*-* } .-1 }

fn s(){test!(1,i)}

fn main() {}

