trait Mul<T> {
    type Output;
}
trait Matrix: Mul<<Self as Matrix>::Row, Output = ()> {
    type Row;
    type Transpose: Matrix<Row = Self::Row>;
}
fn is_mul<S, T: Mul<S, Output = ()>>() {}
fn f<T: Matrix>() {
    is_mul::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<
        f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<
        f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<
        f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::
        <f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<f::<>();
// { dg-error "" "" { target *-*-* } .-1 }
}
fn main() {}

