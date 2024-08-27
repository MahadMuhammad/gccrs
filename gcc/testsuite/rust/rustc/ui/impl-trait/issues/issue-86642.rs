static x: impl Fn(&str) -> Result<&str, ()> = move |source| {
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    let res = (move |source| Ok(source))(source);
    let res = res.or((move |source| Ok(source))(source));
    res
};

fn main() {}

