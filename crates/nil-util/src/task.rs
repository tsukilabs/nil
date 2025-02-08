use tauri::async_runtime::{RuntimeHandle, handle};
use tokio::task::JoinHandle;

pub fn spawn<F>(task: F) -> JoinHandle<F::Output>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  let RuntimeHandle::Tokio(handle) = handle();
  handle.spawn(task)
}
