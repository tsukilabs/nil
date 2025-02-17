use axum::extract::FromRef;
use nil_core::World;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, FromRef)]
pub(crate) struct App {
  pub world: WorldState,
}

impl App {
  pub fn new(world: World) -> Self {
    Self { world: WorldState::new(world) }
  }
}

pub(crate) struct WorldState(Arc<RwLock<World>>);

impl WorldState {
  pub fn new(world: World) -> Self {
    Self(Arc::new(RwLock::new(world)))
  }

  pub async fn world<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&World) -> T,
  {
    f(&*self.0.read().await)
  }

  pub async fn world_mut<F, T>(&self, f: F) -> T
  where
    F: FnOnce(&mut World) -> T,
  {
    f(&mut *self.0.write().await)
  }
}

impl Clone for WorldState {
  fn clone(&self) -> Self {
    Self(Arc::clone(&self.0))
  }
}
