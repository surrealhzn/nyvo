mod create;
mod error;
mod parse;

pub use create::*;
pub use error::*;
pub use parse::*;

pub const MAGIC: [u8; 8] = [0xa8, 0x28, b'N', b'y', b'v', b'o', 0x28, 0xa8];
