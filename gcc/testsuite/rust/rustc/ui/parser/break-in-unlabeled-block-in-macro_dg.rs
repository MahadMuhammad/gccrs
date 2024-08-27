macro_rules! foo {
    () => {
        break (); // { dg-error ".E0268." "" { target *-*-* } }
    };
    ($e: expr) => {
        break $e; // { dg-error ".E0268." "" { target *-*-* } }
    };
    (stmt $s: stmt) => {
        $s
    };
    (@ $e: expr) => {
        { break $e; } // { dg-error ".E0268." "" { target *-*-* } }
    };
    (=> $s: stmt) => {
        { $s }
    };
}

fn main() {
    {
        foo!();
    }
    {
        foo!(());
    }
    {
        foo!(stmt break ()); // { dg-error ".E0268." "" { target *-*-* } }
    }
    {
        foo!(@ ());
    }
    {
        foo!(=> break ()); // { dg-error ".E0268." "" { target *-*-* } }
    }
    {
        macro_rules! bar {
            () => {
                break () // { dg-error ".E0268." "" { target *-*-* } }
            };
        }
        bar!()
    }
}

