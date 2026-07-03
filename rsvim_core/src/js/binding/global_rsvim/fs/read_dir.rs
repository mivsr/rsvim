//! Read directory APIs.

use crate::is_v8_str;
use crate::js;
use crate::js::JsFuture;
use crate::js::JsRuntime;
use crate::js::binding;
use crate::js::converter::*;
use crate::js::pending;
use crate::js::resource::ResourceId;
use crate::js::resource::ResourceTableArc;
use crate::prelude::*;

pub fn fs_read_dir_s(
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
    let rid = postcard::from_bytes::<ResourceId>(&result).unwrap();
    let rid = Into::<i32>::into(rid);
    let rid = rid.to_v8(scope);

    self.promise.open(scope).resolve(scope, rid).unwrap();
  }
}

fn _get_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> String {
  debug_assert!(args.length() == 1);
  debug_assert!(is_v8_str!(args.get(0)));
  let filename = args.get(0).to_rust_string_lossy(scope);
  trace!("RsvimFs readDir filename:{:?}", filename);
  filename
}

/// `Rsvim.fs.readDir` API.
pub fn read_dir_async<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _get_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let read_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsReadDirFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_read_dir(
    &mut state,
    task_id,
    Path::new(&filename),
    Box::new(read_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.readDirSync` API.
pub fn read_dir_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _get_args(scope, args);

  let state_rc = JsRuntime::state(scope);
  let resource_table = state_rc.borrow().resource_table.clone();

  match fs_read_dir_s(resource_table, Path::new(&filename)) {
    Ok(rd_rid) => {
      let rd_rid = Into::<i32>::into(rd_rid);
      let rd_rid = rd_rid.to_v8(scope);
      rv.set(rd_rid);
    }
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}
