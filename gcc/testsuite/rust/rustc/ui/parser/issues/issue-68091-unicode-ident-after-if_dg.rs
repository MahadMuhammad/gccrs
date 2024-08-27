macro_rules! x {
    ($($c:tt)*) => {
        $($c)ö* {}
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

fn main() {
    x!(if);
}

