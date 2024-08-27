#![deny(single_use_lifetimes)]

pub enum Data<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl<'a> Data<'a> {
    pub fn get<'b: 'a>(&'b self) -> &'a str {
// { dg-error "" "" { target *-*-* } .-1 }
        match &self {
            Self::Borrowed(val) => val,
            Self::Owned(val) => &val,
        }
    }
}

fn main() {}

