struct State;

fn once(_: impl FnOnce()) {}

fn fill_memory_blocks_mt(state: &mut State) {
    loop {
        once(move || {
// { dg-error ".E0382." "" { target *-*-* } .-1 }
            fill_segment(state);
        });
    }
}

fn fill_segment(_: &mut State) {}

fn main() {}

