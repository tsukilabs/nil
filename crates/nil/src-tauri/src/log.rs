use anyhow::Result;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

const TIMESTAMP: &str = "%F %T%.3f";

pub fn setup() -> Result<()> {
  let filter = EnvFilter::builder()
    .from_env()?
    .add_directive("nil=trace".parse()?)
    .add_directive("nil_client=trace".parse()?)
    .add_directive("nil_core=trace".parse()?)
    .add_directive("nil_server=trace".parse()?);

  let stderr = Layer::default()
    .with_ansi(true)
    .with_timer(ChronoLocal::new(TIMESTAMP.into()))
    .with_writer(std::io::stderr)
    .pretty();

  set_global_default(Registry::default().with(stderr).with(filter))?;

  Ok(())
}
