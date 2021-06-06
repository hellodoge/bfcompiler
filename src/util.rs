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


#[cfg(test)]
mod tests {

    #[test]
    fn get_unique_test() {
        use std::thread;
        use std::sync::mpsc;
        use std::collections::HashSet;
        use crate::util::get_unique;

        const THREADS_COUNT: usize = 128;
        const GET_UNIQUE_CALLS_PER_THREAD: usize = 1024;

        let (tx_original, rx) = mpsc::channel();

        for i in 1..=THREADS_COUNT {
            let tx = tx_original.clone();
            thread::spawn(move || {
                for _ in 0..GET_UNIQUE_CALLS_PER_THREAD {
                    tx.send(get_unique());
                }
            });
        }

        std::mem::drop(tx_original);

        let mut messages_set = HashSet::new();
        let mut messages_count: usize = 0;
        for message in rx {
            messages_count += 1;
            let is_unique = messages_set.insert(message);
            assert_eq!(is_unique, true, "get_unique() return value is not unique");
        }

        assert_eq!(messages_count, THREADS_COUNT * GET_UNIQUE_CALLS_PER_THREAD);
    }
}