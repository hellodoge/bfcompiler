use std::sync::atomic;

/// used to get unique identifiers for labels
/// atomic used to achieve safe concurrent compilation
pub(crate) fn get_unique() -> String {
    static mut GET_LABEL_CONTEXT: atomic::AtomicUsize = atomic::AtomicUsize::new(0);
    let unique: usize;
    unsafe {
        unique = GET_LABEL_CONTEXT.fetch_add(1, atomic::Ordering::Relaxed);
    }
    return unique.to_string()
}