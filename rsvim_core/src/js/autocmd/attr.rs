//! Autocmd attributes.

use compact_str::CompactString;
use compact_str::ToCompactString;

#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  derive_builder::Builder,
  rsvim_macro::ToV8,
  rsvim_macro::FromV8,
)]
pub struct CommandAttributes {
  #[builder(default = "".to_compact_string())]
  pub event: CompactString,

  #[builder(default = "".to_compact_string())]
  pub pattern: CompactString,
}
