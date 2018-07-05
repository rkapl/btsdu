extern crate btrfs_send_parse as bf;
use std::io;

fn main() {
    let mut input = io::stdin();
    let mut parser = bf::BtrfsReader::new(&mut input).unwrap();
    let opts = bf::CommandPrintOptions::default();
    println!("Stream version: {}", parser.version());
    loop {
        let cmd = match parser.read_command().unwrap() {
            None => break,
            Some(x) => x
        };
        cmd.print(&mut io::stdout(), &opts).unwrap();
    }
}
