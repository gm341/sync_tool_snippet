mod network_manager;

// Placeholder 1MB size for network chunks.
// Would be configurable.
const CONFIGURED_CHUNK_SIZE: usize = 1024*1024;

// Placeholder error type for Results
pub enum GenericErr {
  Error
}

impl From<std::io::Error> for GenericErr {
  fn from(_: std::io::Error) -> Self {
    GenericErr::Error
  }
}
