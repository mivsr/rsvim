//! Read directory APIs.

use crate::js::JsFuture;
use crate::js::binding;
use crate::js::converter::*;
use crate::js::resource::ResourceId;
use crate::js::resource::ResourceTableArc;
use crate::prelude::*;

pub fn fs_read_dir(
  resource_table: ResourceTableArc,
  path: &Path,
) -> TheResult<ResourceId> {
  match std::fs::read_dir(path) {
    Ok(rd) => {
      let mut resource_table = lock!(resource_table);
      Ok(resource_table.add_read_dir(rd))
    }
    Err(e) => Err(TheErr::ReadDirectoryFailed(path.to_path_buf(), e)),
  }
}

pub struct FsReadDirFuture {
  pub promise: v8::Global<v8::PromiseResolver>,
  pub maybe_result: Option<TheResult<Vec<u8>>>,
}

impl JsFuture for FsReadDirFuture {
  fn run(&mut self, scope: &mut v8::PinScope) {
    trace!("|FsReadDirFuture|");

    let result = self.maybe_result.take().unwrap();

    // Handle when something goes wrong with it.
    if let Err(e) = result {
      let message = v8::String::new(scope, &e.to_string()).unwrap();
      let exception = v8::Exception::error(scope, message);
      binding::set_exception_code(scope, exception, &e);
      self.promise.open(scope).reject(scope, exception);
      return;
    }

    // Otherwise, resolve the promise passing the result.
    let result = result.unwrap();
    let readdir_rid = postcard::from_bytes::<ResourceId>(&result).unwrap();
    let readdir_rid = Into::<i32>::into(readdir_rid);
    let readdir_rid = readdir_rid.to_v8(scope);

    self
      .promise
      .open(scope)
      .resolve(scope, readdir_rid)
      .unwrap();
  }
}
