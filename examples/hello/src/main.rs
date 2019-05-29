use std::io::{Read, Write};
use log::info;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use localstoragefs::fs;
    } else {
        use std::fs;
    }
}

struct Logger {}
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn flush(&self) { }

    fn log(&self, record: &log::Record) {
        println!("{}", record.args());
    }

}

fn main() {
    log::set_boxed_logger(Box::new(Logger{})).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

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
