pub fn noop() -> Box<dyn FnMut()> {
    Box::new(|| {})
}
