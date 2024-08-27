trait MyTrait {
    #[doc = MyTrait]
// { dg-error "" "" { target *-*-* } .-1 }
    fn myfun();
}

fn main() {}

