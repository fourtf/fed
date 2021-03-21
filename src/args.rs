use std::sync::atomic::AtomicBool;

pub static VERBOSE_LSP: AtomicBool = AtomicBool::new(false);
pub static VERBOSE_LSP_STDERR: AtomicBool = AtomicBool::new(false);

