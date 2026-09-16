//! Autocmd event.

use crate::js::converter::*;
use compact_str::CompactString;
use compact_str::ToCompactString;

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
}

impl FromV8 for AutoCmdEvent {
  fn from_v8<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    value: v8::Local<'s, v8::Value>,
  ) -> Self {
    debug_assert!(value.is_string() || value.is_string_object());
    let nargs = value.to_string(scope).unwrap().to_rust_string_lossy(scope);
    AutoCmdEvent::from_str(&nargs).unwrap()
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
