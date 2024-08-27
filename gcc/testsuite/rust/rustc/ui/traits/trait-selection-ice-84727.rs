// ICE Where clause `Binder(..)` was applicable to `Obligation(..)` but now is not
// issue: rust-lang/rust#84727

struct Cell<Fg, Bg = Fg> {
    foreground: Color<Fg>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    background: Color<Bg>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

trait Over<Bottom, Output> {
    fn over(self) -> Output;
}

impl<TopFg, TopBg, BottomFg, BottomBg, NewFg, NewBg>
    Over<Cell<BottomFg, BottomBg>, Cell<NewFg, NewBg>> for Cell<TopFg, TopBg>
where
    Self: Over<Color<BottomBg>, Cell<NewFg>>,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
    fn over(self) -> Cell<NewFg> {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        self.over();
    }
}

impl<'b, TopFg, TopBg, BottomFg, BottomBg> Over<&Cell<BottomFg, BottomBg>, ()>
    for Cell<TopFg, TopBg>
where
    Cell<TopFg, TopBg>: Over<Cell<BottomFg>, Cell<BottomFg>>,
{
    fn over(self) -> Cell<NewBg> {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
        self.over();
    }
}

pub fn main() {}

