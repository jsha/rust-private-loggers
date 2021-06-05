struct StdoutLog();

unsafe impl Send for StdoutLog {}
unsafe impl Sync for StdoutLog {}

impl log::Log for StdoutLog {
    fn enabled(&self, _metadata: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        println!("two: {}", record.args())
    }
    fn flush(&self) {
        panic!()
    }
}

#[no_mangle]
pub extern "C" fn saygoodbye() {
    log::set_max_level(log::LevelFilter::Debug);
    log::set_boxed_logger(Box::new(StdoutLog())).ok();
    log::info!("goodbye");
}
