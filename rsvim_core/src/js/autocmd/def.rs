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

#[derive_where::derive_where(Debug)]
#[derive(Clone, rsvim_macro::ToV8, rsvim_macro::RcPtr)]
pub struct AutoCmdDefinition {
  pub id: CompactString,
  #[derive_where(skip)]
  pub callback: AutoCmdCallback,
  pub attributes: CommandAttributes,
  pub options: CommandOptions,
}
