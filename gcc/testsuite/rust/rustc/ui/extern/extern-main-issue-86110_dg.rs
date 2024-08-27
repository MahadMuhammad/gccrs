// missing and missing2 exist to make sure that the error only happens on a `main` declaration
extern "C" {
    fn missing();
    fn main();
// { dg-error "" "" { target *-*-* } .-1 }
    fn missing2();
}

