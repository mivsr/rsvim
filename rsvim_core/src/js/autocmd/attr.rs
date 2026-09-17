//! Autocmd attributes.

use crate::js::converter::*;
use compact_str::CompactString;
use compact_str::ToCompactString;
use regress::Regex;
use std::str::FromStr;

#[derive(
  Debug,
  Copy,
  Clone,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  strum_macros::Display,
  strum_macros::EnumString,
)]
pub enum AutoCmdEvent {
  #[strum(serialize = "VimEnter")]
  /// After editor enter
  VimEnter,

  #[strum(serialize = "VimLeave")]
  /// Just before editor exit
  VimLeave,

  #[strum(serialize = "VimLeavePre")]
  /// Just before all exit operations before editor exit
  VimLeavePre,

  #[strum(serialize = "VimResized")]
  /// After editor resized
  VimResized,
}

impl FromV8 for AutoCmdEvent {
  fn from_v8<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    value: v8::Local<'s, v8::Value>,
  ) -> Self {
    debug_assert!(value.is_string() || value.is_string_object());
    let event = value.to_string(scope).unwrap().to_rust_string_lossy(scope);
    AutoCmdEvent::from_str(&event).unwrap()
  }
}

impl ToV8 for AutoCmdEvent {
  fn to_v8<'s>(
    &self,
    scope: &mut v8::PinScope<'s, '_>,
  ) -> v8::Local<'s, v8::Value> {
    self.to_string().to_v8(scope)
  }
}

#[derive(
  Debug, Clone, derive_builder::Builder, rsvim_macro::ToV8, rsvim_macro::FromV8,
)]
pub struct AutoCmdAttributes {
  #[builder]
  pub event: AutoCmdEvent,

  #[builder(default = None)]
  pub pattern: Option<CompactString>,

  #[builder(default = None)]
  #[ignored_field]
  pub pattern_regex: Option<Regex>,

  #[builder(default = None)]
  pub description: Option<CompactString>,
}

impl PartialEq for AutoCmdAttributes {
  fn eq(&self, other: &Self) -> bool {
    // Ignore "pattern_regex"
    self.event == other.event && self.pattern == other.pattern
  }
}

impl Eq for AutoCmdAttributes {}
