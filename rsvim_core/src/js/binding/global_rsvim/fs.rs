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
use crate::js::binding::global_rsvim::fs::link::fs_link;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirFuture;
use crate::js::binding::global_rsvim::fs::mkdir::FsMkdirOptions;
use crate::js::binding::global_rsvim::fs::mkdir::fs_mkdir;
use crate::js::binding::global_rsvim::fs::read_dir::FsReadDirFuture;
use crate::js::binding::global_rsvim::fs::read_dir::fs_read_dir;
use crate::js::binding::global_rsvim::fs::read_text_file::FsReadTextFileFuture;
use crate::js::binding::global_rsvim::fs::read_text_file::fs_read_text_file;
use crate::js::binding::global_rsvim::fs::stat::FsStatFuture;
use crate::js::binding::global_rsvim::fs::stat::fs_lstat;
use crate::js::binding::global_rsvim::fs::stat::fs_stat;
use crate::js::binding::global_rsvim::fs::symlink::FsSymlinkFuture;
use crate::js::binding::global_rsvim::fs::symlink::FsSymlinkOptions;
use crate::js::binding::global_rsvim::fs::symlink::fs_symlink;
use crate::js::converter::*;
use crate::js::pending;
use crate::js::resource::ResourceId;
use crate::prelude::*;
use itertools::Itertools;
use std::str::FromStr;

fn _read_text_file_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> String {
  debug_assert!(args.length() == 1);
  debug_assert!(is_v8_str!(args.get(0)));
  let filename = args.get(0).to_rust_string_lossy(scope);
  trace!("RsvimFs readTextFile filename:{:?}", filename);
  filename
}

/// `Rsvim.fs.readTextFile` API.
pub fn read_text_file<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _read_text_file_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let read_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsReadTextFileFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_read_text_file(
    &mut state,
    task_id,
    Path::new(&filename),
    Box::new(read_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.readTextFileSync` API.
pub fn read_text_file_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _read_text_file_args(scope, args);

  match fs_read_text_file(Path::new(&filename)) {
    Ok(data) => {
      let data = v8::String::new(scope, &data).unwrap();

      rv.set(data.into());
    }
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}

fn _lstat_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> String {
  debug_assert!(args.length() == 1);
  debug_assert!(is_v8_str!(args.get(0)));
  let filename = args.get(0).to_rust_string_lossy(scope);
  trace!("RsvimFs lstat filename:{:?}", filename);
  filename
}

/// `Rsvim.fs.lstat` API.
pub fn lstat<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _lstat_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let stat_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsStatFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_stat(
    &mut state,
    task_id,
    false,
    Path::new(&filename),
    Box::new(stat_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.lstatSync` API.
pub fn lstat_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _lstat_args(scope, args);

  match fs_lstat(Path::new(&filename)) {
    Ok(info) => {
      let info = info.to_v8(scope);
      rv.set(info);
    }
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}

fn _stat_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> String {
  debug_assert!(args.length() == 1);
  debug_assert!(is_v8_str!(args.get(0)));
  let filename = args.get(0).to_rust_string_lossy(scope);
  trace!("RsvimFs stat filename:{:?}", filename);
  filename
}

/// `Rsvim.fs.stat` API.
pub fn stat<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _stat_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let stat_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsStatFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_stat(
    &mut state,
    task_id,
    true,
    Path::new(&filename),
    Box::new(stat_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.statSync` API.
pub fn stat_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let filename = _stat_args(scope, args);

  match fs_stat(Path::new(&filename)) {
    Ok(info) => {
      let info = info.to_v8(scope);
      rv.set(info);
    }
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}

fn _symlink_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> (
  /* oldpath */ String,
  /* newpath */ String,
  /* options */ FsSymlinkOptions,
) {
  debug_assert!(args.length() == 3);
  debug_assert!(is_v8_str!(args.get(0)));
  let oldpath = args.get(0).to_rust_string_lossy(scope);
  debug_assert!(is_v8_str!(args.get(1)));
  let newpath = args.get(1).to_rust_string_lossy(scope);
  debug_assert!(is_v8_str!(args.get(2)));
  let options = args.get(2).to_rust_string_lossy(scope);
  let options = FsSymlinkOptions::from_str(&options).unwrap();
  trace!(
    "RsvimFs symlink oldpath:{:?},newpath:{:?},options:{:?}",
    oldpath, newpath, options
  );
  (oldpath, newpath, options)
}

/// `Rsvim.fs.symlink` API.
pub fn symlink<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (oldpath, newpath, options) = _symlink_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let link_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsSymlinkFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_symlink(
    &mut state,
    task_id,
    Path::new(&oldpath),
    Path::new(&newpath),
    options,
    Box::new(link_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.symlinkSync` API.
pub fn symlink_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (oldpath, newpath, options) = _symlink_args(scope, args);

  match fs_symlink(Path::new(&oldpath), Path::new(&newpath), options) {
    Ok(_) => rv.set_undefined(),
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}

fn _link_args<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
) -> (/* oldpath */ String, /* newpath */ String) {
  debug_assert!(args.length() == 2);
  debug_assert!(is_v8_str!(args.get(0)));
  let oldpath = args.get(0).to_rust_string_lossy(scope);
  debug_assert!(is_v8_str!(args.get(1)));
  let newpath = args.get(1).to_rust_string_lossy(scope);
  trace!("RsvimFs link oldpath:{:?},newpath:{:?}", oldpath, newpath);
  (oldpath, newpath)
}

/// `Rsvim.fs.link` API.
pub fn link<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (oldpath, newpath) = _link_args(scope, args);

  let promise_resolver = v8::PromiseResolver::new(scope).unwrap();
  let promise = promise_resolver.get_promise(scope);

  let state_rc = JsRuntime::state(scope);
  let link_cb = {
    let promise = v8::Global::new(scope, promise_resolver);
    let state_rc = state_rc.clone();
    move |maybe_result: Option<TheResult<Vec<u8>>>| {
      let fut = FsLinkFuture {
        promise: promise.clone(),
        maybe_result,
      };
      let mut state = state_rc.borrow_mut();
      state.pending_futures.push(Box::new(fut));
    }
  };

  let mut state = state_rc.borrow_mut();
  let task_id = js::TaskId::next();
  pending::create_fs_link(
    &mut state,
    task_id,
    Path::new(&oldpath),
    Path::new(&newpath),
    Box::new(link_cb),
  );

  rv.set(promise.into());
}

/// `Rsvim.fs.linkSync` API.
pub fn link_sync<'s>(
  scope: &mut v8::PinScope<'s, '_>,
  args: v8::FunctionCallbackArguments<'s>,
  mut rv: v8::ReturnValue,
) {
  let (oldpath, newpath) = _link_args(scope, args);

  match fs_link(Path::new(&oldpath), Path::new(&newpath)) {
    Ok(_) => rv.set_undefined(),
    Err(e) => {
      binding::throw_exception(scope, &e);
    }
  }
}

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
