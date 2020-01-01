//! Thread local runtime context
use crate::runtime::{io, Spawner, time};

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
    io_handle: io::Handle,

    /// Handles to the time drivers
    time_handle: time::Handle,

    /// Source of `Instant::now()`
    clock: Option<time::Clock>,
}

#[cfg(all(feature = "io-driver", not(loom)))]
pub(crate) fn io_handle() -> io::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref ctx) => ctx.io_handle.clone(),
        None => None,
    })
}

#[cfg(all(feature = "time", not(loom)))]
pub(crate) fn time_handle() -> time::Handle {
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
pub(crate) fn clock() -> Option<time::Clock> {
    CONTEXT.with(
        |ctx| match ctx.borrow().as_ref().map(|ctx| ctx.clock.clone()) {
            Some(Some(clock)) => Some(clock),
            _ => None,
        },
    )
}

// TODO: remove once testing time can be done with public APIs only.
#[cfg(all(feature = "test-util", feature = "time", test))]
pub(crate) fn with_time<F, R>(time: time::Handle, clock: time::Clock, f: F) -> R
where
    F: FnOnce() -> R
{
    struct Reset(Option<ThreadContext>);

    impl Drop for Reset {
        fn drop(&mut self) {
            CONTEXT.with(|cell| {
                *cell.borrow_mut() = self.0.take();
            });
        }
    }

    let _reset = CONTEXT.with(|cell| {
        let mut ctx = cell.borrow_mut();

        let prev = ctx.take();
        let mut next = prev.clone().unwrap_or_else(|| ThreadContext {
            spawner: Spawner::Shell,
            io_handle: Default::default(),
            time_handle: Default::default(),
            clock: None,
        });

        next.time_handle = time;
        next.clock = Some(clock);

        *ctx = Some(next);

        Reset(prev)
    });

    f()
}

impl ThreadContext {
    /// Construct a new [`ThreadContext`]
    ///
    /// [`ThreadContext`]: struct.ThreadContext.html
    pub(crate) fn new(
        spawner: Spawner,
        io_handle: io::Handle,
        time_handle: time::Handle,
        clock: Option<time::Clock>,
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
