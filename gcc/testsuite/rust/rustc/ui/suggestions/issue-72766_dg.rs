// { dg-additional-options "-frust-edition=2018" }
//@ incremental

pub struct SadGirl;

impl SadGirl {
    pub async fn call(&self) -> Result<(), ()> {
        Ok(())
    }
}

async fn async_main() -> Result<(), ()> {
    // should be `.call().await?`
    SadGirl {}.call()?; // { dg-error ".E0277." "" { target *-*-* } }
    Ok(())
}

fn main() {
    let _ = async_main();
}

