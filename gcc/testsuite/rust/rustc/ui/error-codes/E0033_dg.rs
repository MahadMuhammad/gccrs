trait SomeTrait {
    fn foo(&self);
}
struct S;
impl SomeTrait for S {
    fn foo(&self) {}
}
fn main() {
    let trait_obj: &dyn SomeTrait = &S;

    let &invalid = trait_obj;
// { dg-error ".E0033." "" { target *-*-* } .-1 }
}

