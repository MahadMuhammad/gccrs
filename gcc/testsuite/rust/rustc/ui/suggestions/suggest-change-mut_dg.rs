#![allow(warnings)]

use std::io::{BufRead, BufReader, Read, Write};

fn issue_81421<T: Read + Write>(mut stream: T) { // { help "" "" { target *-*-* } }
    let initial_message = format!("Hello world");
    let mut buffer: Vec<u8> = Vec::new();
    let bytes_written = stream.write_all(initial_message.as_bytes());
    let flush = stream.flush();

    loop {
        let mut stream_reader = BufReader::new(&stream);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
// { help ".E0277." "" { target *-*-* } .-3 }
        stream_reader.read_until(b'\n', &mut buffer).expect("Reading into buffer failed");
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    }
}

fn main() {}

