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

    let filename = "hello.txt";

    info!("opening {}", filename);
    if let Ok(mut g) = fs::File::open(filename) {
        let mut contents = String::new();
        info!("reading {}", filename);
        g.read_to_string(&mut contents).unwrap();
        info!("read data = {:?}", contents);
    } else {
        info!("creating {}", filename);
        let mut f = fs::File::create(filename).unwrap();
        info!("writing {}", filename);
        f.write_all(b"Hello, world!").unwrap();
    }
}
