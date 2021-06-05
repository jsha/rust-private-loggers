struct StdoutLog();

unsafe impl Send for StdoutLog {}
unsafe impl Sync for StdoutLog {}

impl ackbar::Log for StdoutLog {
    fn enabled(&self, _metadata: &ackbar::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &ackbar::Record<'_>) {
        println!("one: {}", record.args())
    }
    fn flush(&self) {
        panic!()
    }
}

#[no_mangle]
pub extern "C" fn sayhello() {
    ackbar::set_max_level(ackbar::LevelFilter::Debug);
    ackbar::set_boxed_logger(Box::new(StdoutLog())).ok();
    ackbar::info!("hello");
}
