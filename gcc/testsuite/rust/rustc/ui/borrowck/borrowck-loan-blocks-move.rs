fn take(_v: Box<isize>) {
}





fn box_imm() {
    let v = Box::new(3);
    let w = &v;
    take(v); // { dg-error ".E0505." "" { target *-*-* } }
    w.use_ref();
}

fn main() {
}

trait Fake { fn use_mut(&mut self) { } fn use_ref(&self) { }  }
impl<T> Fake for T { }

