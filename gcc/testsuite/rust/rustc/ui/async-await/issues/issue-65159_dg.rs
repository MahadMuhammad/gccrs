// Regression test for #65159. We used to ICE.
//
// { dg-additional-options "-frust-edition=2018" }

async fn copy() -> Result<()>
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { dg-error ".E0107." "" { target *-*-* } .-2 }
{
    Ok(())
}

fn main() {}

