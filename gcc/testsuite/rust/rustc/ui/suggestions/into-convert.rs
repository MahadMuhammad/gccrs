use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

fn main() {
    let x: A = B;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let y: Arc<Path> = PathBuf::new();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }

    let z: AtomicU32 = 1;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

struct A;
struct B;

impl From<B> for A {
    fn from(_: B) -> Self {
        A
    }
}

