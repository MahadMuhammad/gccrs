macro_rules! number {
    (neg false, $self:ident) => { $self };
    ($signed:tt => $ty:ty;) => {
        number!(neg $signed, $self);
// { dg-error "" "" { target *-*-* } .-1 }
    };
}

number! { false => u8; }

fn main() {}

