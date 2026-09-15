//! Autocmd runtime context.

use crate::buf::BufferId;
use crate::ui::tree::NodeId;
use compact_str::CompactString;
use compact_str::ToCompactString;

#[derive(
  Debug, Clone, PartialEq, Eq, derive_builder::Builder, rsvim_macro::ToV8,
)]
pub struct AutoCmdContext {
  #[builder(default = "".to_compact_string())]
  pub name: CompactString,

  #[builder(default = vec![])]
  pub pattern: Vec<CompactString>,

  #[builder(default = None)]
  pub buffer_id: Option<BufferId>,

  #[builder(default = None)]
  pub window_id: Option<NodeId>,
}
