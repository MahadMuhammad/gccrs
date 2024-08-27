fn use_iterator<I>(itr: I)
where
    I: IntoIterator<Item = i32>,
{
}

fn pass_iterator<I>(i: &dyn IntoIterator<Item = i32, IntoIter = I>)
where
    I: Iterator<Item = i32>,
{
    use_iterator(i);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

