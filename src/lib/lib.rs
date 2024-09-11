pub mod server;

pub const BUFFER_SIZE: usize = 65536;

pub type Error = Box<dyn std::error::Error + Send + Sync>;