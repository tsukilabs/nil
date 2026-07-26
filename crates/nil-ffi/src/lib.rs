// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

#![feature(nonpoison_rwlock, sync_nonpoison)]
#![expect(clippy::missing_safety_doc)]

mod result;
mod string;

use crate::result::write;
use crate::string::into_c_string;
use nil_client::Client;
use std::ffi::{CStr, CString, c_char};
use std::ptr;
use std::sync::LazyLock;
use std::sync::nonpoison::RwLock;

pub use result::{FfiResult, Status};

static CLIENT: LazyLock<RwLock<Client>> = LazyLock::new(RwLock::default);

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_client_version() -> *mut c_char {
  into_c_string(nil_client::VERSION)
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_ffi_version() -> *mut c_char {
  into_c_string(env!("CARGO_PKG_VERSION"))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn callofnil_free_str(ptr: *mut c_char) {
  if !ptr.is_null() {
    drop(unsafe { CString::from_raw(ptr) });
  }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn callofnil_set_user_agent(
  user_agent: *const c_char,
  out: *mut *mut c_char,
) -> Status {
  if user_agent.is_null() {
    Status::ERR_NULL_POINTER
  } else {
    let user_agent = unsafe { CStr::from_ptr(user_agent) };
    match user_agent.to_str() {
      Ok(user_agent) => unsafe {
        write(user_agent, out, |value| {
          CLIENT.write().set_user_agent(value);
        })
      },
      Err(_) => Status::ERR_INVALID_UTF8,
    }
  }
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_user_agent() -> *mut c_char {
  into_c_string(CLIENT.read().user_agent())
}

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_world() -> *mut c_char {
  match CLIENT.read().world() {
    Some(world) => into_c_string(world.to_string()),
    None => ptr::null_mut(),
  }
}
