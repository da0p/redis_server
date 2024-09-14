pub mod server;

pub const BUFFER_SIZE: usize = 4096;

pub type Error = Box<dyn std::error::Error + Send + Sync>;