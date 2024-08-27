// Tests the suggestion to reborrow the first move site
// when we move then borrow a `&mut` ref.

struct State;

impl IntoIterator for &mut State {
    type IntoIter = std::vec::IntoIter<()>;
    type Item = ();

    fn into_iter(self) -> Self::IntoIter {
        vec![].into_iter()
    }
}

fn once(f: impl FnOnce()) {}

fn fill_memory_blocks_mt(state: &mut State) {
    for _ in state {}
// { help "" "" { target *-*-* } .-1 }
    fill_segment(state);
// { dg-error ".E0382." "" { target *-*-* } .-1 }
}

fn fill_segment(state: &mut State) {}

fn main() {}

