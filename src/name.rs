pub struct Name {
    pub text: String,
    pub unique: usize,
}

static COUNTER: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl Name {
    pub fn new(text: String) -> Self {
        Self {
            text,
            unique: COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}
