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

use crate::is_v8_str;
use crate::js;
use crate::js::JsRuntime;
use crate::js::binding;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirFuture;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirOptions;
use crate::js::binding::global_rsvim::fs::mkdir::fs_mkdir;
use crate::js::converter::*;
use crate::js::pending;
use crate::prelude::*;
use std::str::FromStr;

