trait MyIterator : Iterator {}

fn main() {
    let _ = MyIterator::next;
}
// { dg-error ".E0191." "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }

