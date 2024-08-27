trait From<Src> {
    type Result;

    fn from(src: Src) -> Self::Result;
}

trait To {
    fn to<Dst>(
        self // { dg-error ".E0277." "" { target *-*-* } }
    ) -> <Dst as From<Self>>::Result where Dst: From<Self> { // { dg-error ".E0277." "" { target *-*-* } }
        From::from(self) // { dg-error ".E0277." "" { target *-*-* } }
    }
}

fn main() {}

