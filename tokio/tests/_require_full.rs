#![cfg(any(not(feature = "full", feature = "test-util")))]
compile_error!("run main Tokio tests with `--all-features`");
