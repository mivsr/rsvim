//! Resource.

pub mod file;
pub mod read_dir;
pub mod text_decoder;

use crate::prelude::*;
use child_process::ChildProcessResource;
use child_process::ChildProcessStderrResource;
use child_process::ChildProcessStdinResource;
use child_process::ChildProcessStdoutResource;
use file::FileResource;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::Mutex;
use text_decoder::TextDecoderResource;

// ResourceId start from 1.
#[derive(
  Copy, Clone, rsvim_macro::IncrementalId, serde::Serialize, serde::Deserialize,
)]
pub struct ResourceId(#[start_from(1)] i32);

#[derive_where::derive_where(Debug)]
#[derive(Clone)]
/// Resource container.
pub struct ResourceContainer<T> {
  id: ResourceId,

  #[derive_where(skip)]
  data: Arc<Mutex<T>>,
}

impl<T> ResourceContainer<T> {
  pub fn new(data: T) -> Self {
    Self {
      id: ResourceId::next(),
      data: Arc::new(Mutex::new(data)),
    }
  }

  fn id(&self) -> ResourceId {
    self.id
  }

  pub fn data(&self) -> Arc<Mutex<T>> {
    self.data.clone()
  }
}

pub type ChildProcessResource = ResourceContainer<std::process::Child>;
pub type ChildProcessStdinResource =
  ResourceContainer<std::process::ChildStdin>;
pub type ChildProcessStdoutResource =
  ResourceContainer<std::process::ChildStdout>;
pub type ChildProcessStderrResource =
  ResourceContainer<std::process::ChildStderr>;

#[derive(Debug, Clone)]
pub enum Resource {
  File(FileResource),
  TextDecoder(TextDecoderResource),
  ChildProcess(ChildProcessResource),
  ChildProcessStdin(ChildProcessStdinResource),
  ChildProcessStdout(ChildProcessStdoutResource),
  ChildProcessStderr(ChildProcessStderrResource),
}

#[derive(Debug, rsvim_macro::ArcMutexPtr)]
pub struct ResourceTable {
  resources: FoldMap<ResourceId, Resource>,
}

// pub type ResourceTableKeys<'a> =
//   std::collections::btree_map::Keys<'a, ResourceId, Resource>;
// pub type ResourceTableValues<'a> =
//   std::collections::btree_map::Values<'a, ResourceId, Resource>;
// pub type ResourceTableIter<'a> =
//   std::collections::btree_map::Iter<'a, ResourceId, Resource>;

impl ResourceTable {
  pub fn new() -> Self {
    Self {
      resources: FoldMap::new(),
    }
  }

  pub fn add_file(&mut self, data: std::fs::File) -> ResourceId {
    let res = FileResource::new(data);
    let rid = res.id();
    self.resources.insert(res.id(), Resource::File(res));
    rid
  }

  pub fn add_text_decoder(&mut self, data: encoding_rs::Decoder) -> ResourceId {
    let res = TextDecoderResource::new(data);
    let rid = res.id();
    self.resources.insert(res.id(), Resource::TextDecoder(res));
    rid
  }

  pub fn add_child_process(&mut self, data: std::process::Child) -> ResourceId {
    let res = ChildProcessResource::new(data);
    let rid = res.id();
    self.resources.insert(res.id(), Resource::ChildProcess(res));
    rid
  }

  pub fn add_child_process_stdin(
    &mut self,
    data: std::process::ChildStdin,
  ) -> ResourceId {
    let res = ChildProcessStdinResource::new(data);
    let rid = res.id();
    self
      .resources
      .insert(res.id(), Resource::ChildProcessStdin(res));
    rid
  }

  pub fn add_child_process_stdout(
    &mut self,
    data: std::process::ChildStdout,
  ) -> ResourceId {
    let res = ChildProcessStdoutResource::new(data);
    let rid = res.id();
    self
      .resources
      .insert(res.id(), Resource::ChildProcessStdout(res));
    rid
  }

  pub fn add_child_process_stderr(
    &mut self,
    data: std::process::ChildStderr,
  ) -> ResourceId {
    let res = ChildProcessStderrResource::new(data);
    let rid = res.id();
    self
      .resources
      .insert(res.id(), Resource::ChildProcessStderr(res));
    rid
  }

  pub fn get(&self, rid: &ResourceId) -> Option<&Resource> {
    self.resources.get(rid)
  }

  pub fn remove(&mut self, rid: &ResourceId) -> Option<Resource> {
    self.resources.remove(rid)
  }
}
