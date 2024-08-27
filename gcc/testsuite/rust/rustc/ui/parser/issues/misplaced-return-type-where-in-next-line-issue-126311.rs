fn foo<T, K>()
// { help "" "" { target *-*-* } .-1 }
where
    T: Default,
    K: Clone, -> Result<u8, String>
// { dg-error "" "" { target *-*-* } .-1 }
{
    Ok(0)
}

fn main() {}

