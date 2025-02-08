use crate::error::Result;
use crate::state::Nil;
use nil_client::Client;
use tauri::{AppHandle, Manager, State, WebviewWindow, Window, Wry};

pub trait ManagerExt: Manager<Wry> {
  fn nil(&self) -> State<Nil> {
    self.state::<Nil>()
  }

  async fn client<F, T>(&self, f: F) -> Result<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    self.nil().client(f).await
  }
}

impl ManagerExt for AppHandle<Wry> {}
impl ManagerExt for WebviewWindow<Wry> {}
impl ManagerExt for Window<Wry> {}

pub trait WindowExt: Manager<Wry> {
  fn main_window(&self) -> WebviewWindow<Wry> {
    self.get_webview_window("main").unwrap()
  }
}

impl WindowExt for AppHandle<Wry> {}
impl WindowExt for WebviewWindow<Wry> {}
impl WindowExt for Window<Wry> {}
