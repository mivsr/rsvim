//! APIs for `Rsvim.fs` namespace.

pub mod close;
pub mod link;
pub mod mkdir;
pub mod open;
pub mod read;
pub mod read_dir;
pub mod read_file;
pub mod read_text_file;
pub mod stat;
pub mod symlink;
pub mod write;

use crate::is_v8_str;
use crate::js;
use crate::js::JsRuntime;
use crate::js::binding;
use crate::js::binding::global_rsvim::fs::link::FsLinkFuture;
use crate::js::binding::global_rsvim::fs::link::sync_fs_link;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirFuture;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirOptions;
use crate::js::binding::global_rsvim::fs::mkdir::fs_mkdir;
use crate::js::binding::global_rsvim::fs::symlink::FsSymlinkFuture;
use crate::js::binding::global_rsvim::fs::symlink::FsSymlinkOptions;
use crate::js::binding::global_rsvim::fs::symlink::fs_symlink;
use crate::js::converter::*;
use crate::js::pending;
use crate::prelude::*;
use std::str::FromStr;

fn _mkdir_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> (/* path */ String, /* options */ FsMkdirOptions) {
  debug_assert!(args.length() == 2);
  debug_assert!(is_v8_str!(args.get(0)));
  let path = args.get(0).to_rust_string_lossy(scope);
  debug_assert!(args.get(1).is_object());
  let options = FsMkdirOptions::from_v8(scope, args.get(1));
  trace!("RsvimFs mkdir path:{:?},options:{:?}", path, options);
  (path, options)
}

/// `Rsvim.fs.mkdir` API.
pub fn mkdir<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (path, options) = _mkdir_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let link_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsMkdirFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_mkdir(
    &mut state,
    task_id,
    Path::new(&path),
    options,
    Box::new(link_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.mkdirSync` API.
pub fn mkdir_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (path, options) = _mkdir_args(scope, args);

  match fs_mkdir(Path::new(&path), options) {
    Ok(_) => rv.set_undefined(),
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}
