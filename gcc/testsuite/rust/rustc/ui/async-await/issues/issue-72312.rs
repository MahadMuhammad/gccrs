// { dg-additional-options "-frust-edition=2018" }
fn require_static<T: 'static>(val: T) -> T {
    val
}

struct Problem;

impl Problem {
    pub async fn start(&self) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        require_static(async move {
// { dg-error ".E0521." "" { target *-*-* } .-1 }
// { dg-note ".E0521." "" { target *-*-* } .-2 }
// { dg-note ".E0521." "" { target *-*-* } .-3 }
            &self;
        });
    }
}

fn main() {}

