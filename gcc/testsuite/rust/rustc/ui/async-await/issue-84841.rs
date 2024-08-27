// { dg-additional-options "-frust-edition=2018" }

fn main() {

}

async fn foo() {
    // Adding an .await here avoids the ICE
    test()?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

// Removing the const generic parameter here avoids the ICE
async fn test<const N: usize>() {
}

