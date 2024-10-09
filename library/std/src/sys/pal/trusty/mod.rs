//! System bindings for the Trusty OS.

#[path = "../unix/alloc.rs"]
pub mod alloc;
#[path = "../unsupported/args.rs"]
pub mod args;
#[path = "../unsupported/env.rs"]
pub mod env;
#[path = "../unsupported/fs.rs"]
pub mod fs;
#[path = "../unsupported/io.rs"]
pub mod io;
#[path = "../unsupported/net.rs"]
pub mod net;
#[path = "../unsupported/os.rs"]
pub mod os;
#[path = "../unsupported/pipe.rs"]
pub mod pipe;
#[path = "../unsupported/process.rs"]
pub mod process;
pub mod stdio;
#[path = "../unsupported/time.rs"]
pub mod time;
#[path = "../unsupported/thread.rs"]
pub mod thread;
#[path = "../unsupported/common.rs"]
#[deny(unsafe_op_in_unsafe_fn)]
mod common;

pub use common::*;
