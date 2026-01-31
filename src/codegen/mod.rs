pub mod error;
pub mod generator;

pub use error::{CodegenError, CodegenResult};
pub use generator::generate;
