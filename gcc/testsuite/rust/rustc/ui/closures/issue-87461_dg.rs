// Regression test for #87461.

// { dg-additional-options "-frust-edition=2021" }

async fn func() -> Result<u16, u64> {
    let _ = async {
        Err(42u64)
    }.await?;

    Ok(())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

async fn func2() -> Result<u16, u64> {
    Err(42u64)?;

    Ok(())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {
    || -> Result<u16, u64> {
        if true {
            return Err(42u64);
        }
        Ok(())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    };
}

