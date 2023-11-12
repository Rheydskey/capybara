pub use capybara_packet_parser;

mod de;
mod error;
mod ser;
#[cfg(test)]
mod tests;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};
