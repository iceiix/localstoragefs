use std::io::{Read, Write};
use log::info;
use cfg_if::cfg_if;
mod init_logging;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use localstoragefs::fs;
        #[macro_use]
        extern crate stdweb;
    } else {
        use std::fs;
    }
}

fn main() {
    init_logging::init_logging();

    info!("creating");
    let mut f = fs::File::create("hello.txt").unwrap();
    info!("writing");
    f.write_all(b"Hello, world!").unwrap();

    info!("opening");
    let mut g = fs::File::open("hello.txt").unwrap();
    let mut contents = String::new();
    info!("reading");
    g.read_to_string(&mut contents).unwrap();
    info!("read data = {:?}", contents);
}
