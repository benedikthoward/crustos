pub trait TraceBackend {
    fn emit(event: &TraceEvent);
}