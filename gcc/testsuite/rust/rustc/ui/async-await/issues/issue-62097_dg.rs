// { dg-additional-options "-frust-edition=2018" }
async fn foo<F>(fun: F)
where
    F: FnOnce() + 'static
{
    fun()
}

struct Struct;

impl Struct {
    pub async fn run_dummy_fn(&self) {
        foo(|| self.bar()).await;
// { dg-error ".E0521." "" { target *-*-* } .-1 }
// { dg-error ".E0521." "" { target *-*-* } .-2 }
    }

    pub fn bar(&self) {}
}

fn main() {}

