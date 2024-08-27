// { dg-additional-options "-frust-edition=2018" }
#![feature(must_not_suspend)]
#![deny(must_not_suspend)]

async fn other() {}

pub async fn uhoh(m: std::sync::Mutex<()>) {
    let _guard = m.lock().unwrap(); // { dg-error "" "" { target *-*-* } }
    other().await;
}

fn main() {
}

