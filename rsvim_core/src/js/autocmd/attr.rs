//! Autocmd attributes.

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
  #[builder(default = false)]
  pub clear: bool,
}
