use std::thread;
use std::sync::mpsc::channel;

fn bar() {
    let (send, recv) = channel();
    let t = thread::spawn(|| {
        recv.recv().unwrap();
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    });

    send.send(());

    t.join().unwrap();
}

fn main() {}

