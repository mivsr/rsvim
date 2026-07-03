//! APIs for `Rsvim.fs` namespace.

pub mod close;
pub mod link;
pub mod mkdir;
pub mod open;
pub mod read;
pub mod read_dir;
pub mod read_file;
pub mod read_text_file;
pub mod stat;
pub mod symlink;
pub mod write;

pub use close::close_sync;
pub use link::link_async;
pub use link::link_sync;
pub use mkdir::mkdir_async;
pub use mkdir::mkdir_sync;
pub use open::open_async;
pub use open::open_sync;
