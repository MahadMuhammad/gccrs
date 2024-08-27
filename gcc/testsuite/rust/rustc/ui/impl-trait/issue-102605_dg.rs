// { dg-additional-options "-frust-edition=2021" }

async fn foo() -> Result<(), String> {
    Ok(())
}

fn convert_result<T, E>(r: Result<T, E>) -> Option<T> {
    None
}

fn main() -> Option<()> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    convert_result(foo())
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

