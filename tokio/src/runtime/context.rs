//! Thread local runtime context
use crate::runtime::{self, Spawner};
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<ThreadContext>> = RefCell::new(None)
}

/// ThreadContext makes Runtime context accessible to each Runtime thread.
#[derive(Debug, Clone)]
pub(crate) struct ThreadContext {
    /// Handles to the executor.
    spawner: Spawner,

    /// Handles to the I/O drivers
    io_handle: runtime::io::Handle,

    /// Handles to the time drivers
    time_handle: runtime::time::Handle,

    /// Source of `Instant::now()`
    clock: Option<runtime::time::Clock>,
}

#[cfg(all(feature = "io-driver", not(loom)))]
pub(crate) fn io_handle() -> runtime::io::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => ctx.io_handle.clone(),
        None => None,
    })
}

#[cfg(all(feature = "time", not(loom)))]
pub(crate) fn time_handle() -> runtime::time::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => ctx.time_handle.clone(),
        None => None,
    })
}

#[cfg(feature = "rt-core")]
pub(crate) fn spawn_handle() -> Option<Spawner> {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => Some(ctx.spawner.clone()),
        None => None,
    })
}

#[cfg(all(feature = "test-util", feature = "time"))]
pub(crate) fn clock() -> Option<runtime::time::Clock> {
    CONTEXT.with(
        |ctx| match ctx.borrow().as_ref().map(|ctx| ctx.clock.clone()) {
            Some(Some(clock)) => Some(clock),
            _ => None,
        },
    )
}

impl ThreadContext {
    /// Construct a new [`ThreadContext`]
    ///
    /// [`ThreadContext`]: struct.ThreadContext.html
    pub(crate) fn new(
        spawner: Spawner,
        io_handle: runtime::io::Handle,
        time_handle: runtime::time::Handle,
        clock: Option<runtime::time::Clock>,
    ) -> Self {
        ThreadContext {
            spawner,
            io_handle,
            time_handle,
            clock,
        }
    }

    /// Set this [`ThreadContext`] as the current active [`ThreadContext`].
    ///
    /// [`ThreadContext`]: struct.ThreadContext.html
    pub(crate) fn enter(self) -> ThreadContextDropGuard {
        CONTEXT.with(|ctx| {
            let previous = ctx.borrow_mut().replace(self);
            ThreadContextDropGuard { previous }
        })
    }

    /*
    #[cfg(all(feature = "test-util", feature = "time", test))]
    pub(crate) fn with_time_handle(mut self, handle: runtime::time::Handle) -> Self {
        self.time_handle = handle;
        self
    }
    */

    #[cfg(all(feature = "test-util", feature = "time", test))]
    pub(crate) fn with_clock(mut self, clock: runtime::time::Clock) -> Self {
        self.clock.replace(clock);
        self
    }
}

/// [`ThreadContextDropGuard`] will replace the `previous` thread context on drop.
///
/// [`ThreadContextDropGuard`]: struct.ThreadContextDropGuard.html
#[derive(Debug)]
pub(crate) struct ThreadContextDropGuard {
    previous: Option<ThreadContext>,
}

impl Drop for ThreadContextDropGuard {
    fn drop(&mut self) {
        CONTEXT.with(|ctx| match self.previous.clone() {
            Some(prev) => ctx.borrow_mut().replace(prev),
            None => ctx.borrow_mut().take(),
        });
    }
}
