//! Autocmd definition.
//!
//! Here are the key difference between traditional Vim:
//! - We can set an optional name for a "autocmd", the name must be unique just
//!   like command name.
//! - We don't have "autocmd group" (I don't feel it is useful), just manage
//!   all autocmds by IDs/names.

use crate::is_v8_func;
use crate::is_v8_str;
use crate::js::command::opt::*;
use crate::js::converter::*;
use compact_str::CompactString;
use compact_str::ToCompactString;
use std::rc::Rc;

pub type AutoCmdCallback = Rc<v8::Global<v8::Function>>;

// AutoCmdId start from 1.
#[derive(
  Copy, Clone, rsvim_macro::IncrementalId, serde::Serialize, serde::Deserialize,
)]
pub struct AutoCmdId(#[start_from(1)] i32);

#[derive_where::derive_where(Debug)]
#[derive(Clone, rsvim_macro::ToV8, rsvim_macro::RcPtr)]
pub struct AutoCmdDefinition {
  pub id: AutoCmdId,
  pub name: Option<CompactString>, // Optional but unique name for a autocmd
  #[derive_where(skip)]
  pub callback: AutoCmdCallback,
  pub options: CommandOptions,
}
