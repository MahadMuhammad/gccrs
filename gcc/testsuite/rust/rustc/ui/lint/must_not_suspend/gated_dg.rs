//@ check-pass

// { dg-additional-options "-frust-edition=2018" }
#![deny(must_not_suspend)]
// { dg-warning "" "" { target *-*-* } .-1 }

async fn other() {}

pub async fn uhoh(m: std::sync::Mutex<()>) {
    let _guard = m.lock().unwrap();
    other().await;
}

fn main() {
}

