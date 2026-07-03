//! ReadDir resource.

use crate::js::resource::ResourceContainer;
use crate::js::resource::ResourceId;
use std::fs::ReadDir;
use std::sync::Arc;
use std::sync::Mutex;

#[derive_where::derive_where(Debug)]
#[derive(Clone)]
pub struct ReadDirResource {
  id: ResourceId,
  #[derive_where(skip)]
  data: Arc<Mutex<ReadDir>>,
}

impl ReadDirResource {
  pub fn new(data: ReadDir) -> Self {
    Self {
      id: ResourceId::next(),
      data: Arc::new(Mutex::new(data)),
    }
  }

  pub fn data(&self) -> Arc<Mutex<ReadDir>> {
    self.data.clone()
  }
}

impl ResourceContainer for ReadDirResource {
  fn id(&self) -> ResourceId {
    self.id
  }
}
