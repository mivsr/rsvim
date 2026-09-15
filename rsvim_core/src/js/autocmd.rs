//! Vim autocmd, i.e. event/subscription hooks.

pub mod ctx;
pub mod def;

use crate::js::JsFuture;
use crate::js::JsRuntime;
use crate::js::TaskId;
use crate::prelude::*;
use compact_str::CompactString;
use compact_str::ToCompactString;
use ctx::AutoCmdContext;

#[derive(Debug, Clone)]
/// Builtin `:autocmd`
pub struct AutoCmdFuture {
  pub task_id: TaskId,
  pub name: CompactString,
  pub context: AutoCmdContext,
  pub definition: Option<CommandDefinitionRc>,
}
