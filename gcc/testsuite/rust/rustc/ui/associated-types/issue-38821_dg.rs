pub struct Nullable<T: NotNull>(T);

pub trait NotNull {}

pub trait IntoNullable {
    type Nullable;
}

impl<T: NotNull> IntoNullable for T {
    type Nullable = Nullable<T>;
}

impl<T: NotNull> IntoNullable for Nullable<T> {
    type Nullable = Nullable<T>;
}

pub trait Expression {
    type SqlType;
}

pub trait Column: Expression {}

#[derive(Debug, Copy, Clone)]
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-error ".E0277." "" { target *-*-* } .-5 }
// { dg-error ".E0277." "" { target *-*-* } .-6 }
// { dg-error ".E0277." "" { target *-*-* } .-7 }
// { dg-error ".E0277." "" { target *-*-* } .-8 }
// { dg-error ".E0277." "" { target *-*-* } .-9 }
// { dg-error ".E0277." "" { target *-*-* } .-10 }
// { dg-error ".E0277." "" { target *-*-* } .-11 }
// { dg-error ".E0277." "" { target *-*-* } .-12 }
// { dg-error ".E0277." "" { target *-*-* } .-13 }
// { dg-error ".E0277." "" { target *-*-* } .-14 }
// { dg-error ".E0277." "" { target *-*-* } .-15 }
// { dg-error ".E0277." "" { target *-*-* } .-16 }
pub enum ColumnInsertValue<Col, Expr> where
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    Col: Column,
    Expr: Expression<SqlType=<Col::SqlType as IntoNullable>::Nullable>,
{
    Expression(Col, Expr),
    Default(Col),
}

fn main() {}

