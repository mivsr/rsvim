//! File resource.

use crate::js::resource::ResourceContainer;
use crate::js::resource::ResourceId;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;

#[derive_where::derive_where(Debug)]
#[derive(Clone)]
pub struct FileResource {
  id: ResourceId,
  #[derive_where(skip)]
  data: Arc<Mutex<File>>,
}

impl FileResource {
  pub fn new(data: File) -> Self {
    Self {
      id: ResourceId::next(),
      data: Arc::new(Mutex::new(data)),
    }
  }

  pub fn data(&self) -> Arc<Mutex<File>> {
    self.data.clone()
  }
}

impl ResourceContainer for FileResource {
  fn id(&self) -> ResourceId {
    self.id
  }
}
