//! Thread local runtime context
use crate::runtime::{Handle, Spawner, time, io};

use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<Handle>> = RefCell::new(None)
}

/// Set the currently active runtime for the duration of the closure.
pub(crate) fn enter<F>(handle: &Handle, f: F) -> R
where
    F: FnOnce() -> R
{
    struct Reset(Option<Handle>);

    impl Drop for Reset {
        fn drop(&mut self) {
            CONTEXT.with(|ctx| {
                *ctx.borrow_mut() = self.0.take();
            });
        }
    }

    let _reset = CONTEXT.with(|ctx| {
        let ctx = ctx.borrow_mut();
        let prev = ctx.take();

        *ctx = Some(handle.clone());

        Reset(prev)
    });

    f()
}

#[cfg(all(feature = "io-driver", not(loom)))]
pub(crate) fn io_handle() -> io::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref handle) => handle.io_handle.clone(),
        None => None,
    })
}

#[cfg(all(feature = "time", not(loom)))]
pub(crate) fn time_handle() -> time::Handle {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref handle) => handle.time_handle.clone(),
        None => None,
    })
}

#[cfg(feature = "rt-core")]
pub(crate) fn spawn_handle() -> Option<Spawner> {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref handle) => Some(handle.spawner.clone()),
        None => None,
    })
}

#[cfg(all(feature = "test-util", feature = "time"))]
pub(crate) fn clock() -> Option<time::Clock> {
    CONTEXT.with(|ctx| match *ctx.borrow() {
        Some(ref handle) => Some(handle.clock.clone()),
        None => None,
    })
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
        let mut next = prev.clone().unwrap_or_else(|| Handle {
            spawner: Spawner::Shell,
            io_handle: Default::default(),
            time_handle: Default::default(),
            clock: Clock::new(),
            blocking_spawner: Default::default()
        });

        next.time_handle = time;
        next.clock = clock;

        *ctx = Some(next);

        Reset(prev)
    });

    f()
}

/*
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
*/
