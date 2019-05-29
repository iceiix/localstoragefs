use log;

/// A logger supporting both native and wasm
// TODO: is there a module that does this already better?
struct Logger {}
impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool { true }
    fn flush(&self) { }

    #[cfg(target_arch = "wasm32")]
    fn log(&self, record: &log::Record) {
        let s = format!("{}", record.args());
        js! {
            console.log(@{s});
        };
    }


    #[cfg(not(target_arch = "wasm32"))]
    fn log(&self, record: &log::Record) {
        println!("{}", record.args());
    }

}

pub fn init_logging() {
    log::set_boxed_logger(Box::new(Logger{})).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}
