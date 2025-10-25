use std::future::Future;
use std::panic::AssertUnwindSafe;
pub use tokio::spawn;
pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
pub use tokio::task::spawn_local;
pub use tokio::task::JoinHandle;

pub trait BaseAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

// Why AssertUnwindSafe: https://github.com/tokio-rs/tokio/issues/6188
#[derive(Debug)]
pub struct SimpleAsyncRuntime(pub AssertUnwindSafe<tokio::runtime::Runtime>);

impl Default for SimpleAsyncRuntime {
    fn default() -> Self {
        Self(AssertUnwindSafe(
            // TODO: Make the runtime configurable via API
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .global_queue_interval(31)
                .thread_name("frb-tokio-async-runtime")
                .enable_all()
                .build()
                .unwrap(),
        ))
    }
}

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.0.spawn(future)
    }
}

/// Similar to tokio's `spawn_blocking`, except that you need to provide a second argumnet.
///
/// If you are using flutter_rust_bridge, the second argument can be easily provided:
/// Just use `FLUTTER_RUST_BRIDGE_HANDLER.thread_pool()`.
///
/// More formally, the second argument is defined as:
///
/// * When on web: The thread pool you want to use.
/// * When on non-web: Unused, can be anything (since we use Tokio's built-in pool).
pub fn spawn_blocking_with<F, R, TP>(f: F, _thread_pool_on_web: TP) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    tokio::task::spawn_blocking(f)
}
