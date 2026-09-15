//! Autocmd definition.

use crate::is_v8_func;
use crate::is_v8_str;
use crate::js::command::attr::*;
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
pub struct AutoCmdId(#[start_from(1)] usize);

#[derive_where::derive_where(Debug)]
#[derive(Clone, rsvim_macro::ToV8, rsvim_macro::RcPtr)]
pub struct AutoCmdDefinition {
  pub id: AutoCmdId,
  pub name: Option<CompactString>,
  #[derive_where(skip)]
  pub callback: AutoCmdCallback,
  pub attributes: CommandAttributes,
  pub options: CommandOptions,
}
