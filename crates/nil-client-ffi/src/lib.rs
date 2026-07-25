use std::ffi::{CString, c_char};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[unsafe(no_mangle)]
pub extern "C" fn callofnil_ffi_version() -> *mut c_char {
  into_c_string(VERSION)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn callofnil_free_str(ptr: *mut c_char) {
  if !ptr.is_null() {
    drop(unsafe { CString::from_raw(ptr) });
  }
}

fn into_c_string<T>(value: T) -> *mut c_char
where
  T: Into<Vec<u8>>,
{
  CString::new(value)
    .expect("value must not contain nul byte")
    .into_raw()
}
